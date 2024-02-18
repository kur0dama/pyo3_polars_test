### Installation

After cloning repo:
```
python -m venv .venv
maturin develop
source .venv/bin/activate
python
```

### Usage

In Python:
```python
>>> import pyo3_polars_test as pptest

# Get data from Rust HashMap<String, i32>, loaded as dict[str, int]
>>> pptest.get_data_1()
{'a': [1, 2, 3], 'b': [4, 5, 6]}

# Get data from Rust Polars DataFrame, loaded as dict[str, Any]
>>> pptest.get_data_2()
{'letter': ['a', 'b', 'c'], 'number': [1, 2, 3]}
```