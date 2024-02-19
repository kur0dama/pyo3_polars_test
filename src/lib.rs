use std::collections::HashMap;

use polars::datatypes::DataType as PolarsDataType;
use polars::df;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types as pytypes;

fn make_dummy_dataframe() -> DataFrame {
    let dframe = df![
        "letter" => ["a","b","c"],
        "number" => [1i8,2i8,3i8],
    ];
    dframe.unwrap()
}

fn df_to_pydict(py: Python, dframe: DataFrame) -> &pytypes::PyDict {
    let dict: &pytypes::PyDict = pytypes::PyDict::new(py);
    for col in dframe.get_columns() {
        let v = match col.dtype() {
            PolarsDataType::Int8 => col
                .i8()
                .expect("Could not render column as Vec<i8>")
                .into_iter()
                .map(|x| x.to_object(py))
                .collect(),
            PolarsDataType::String => col
                .str()
                .expect("Could not render column as Vec<&str>")
                .into_iter()
                .map(|x| x.to_object(py))
                .collect(),
            _ => vec![],
        };
        let _ = dict.set_item(col.name(), v);
    }
    dict
}

#[pyfunction]
fn get_data_1() -> HashMap<String, Vec<i32>> {
    let mut hmap: HashMap<String, Vec<i32>> = HashMap::new();
    hmap.insert("a".to_owned(), vec![1, 2, 3]);
    hmap.insert("b".to_owned(), vec![4, 5, 6]);
    hmap
}

#[pyfunction]
fn get_data_2(py: Python) -> PyResult<PyObject> {
    let dframe = make_dummy_dataframe();
    let dict = df_to_pydict(py, dframe);
    Ok(dict.into())
}

#[pymodule]
fn pyo3_polars_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_data_1, m)?)?;
    m.add_function(wrap_pyfunction!(get_data_2, m)?)?;
    Ok(())
}
