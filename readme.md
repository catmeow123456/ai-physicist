### set python environment for pyo3
pyenv virtualenv 3.10.0 aiphysicist  
pyenv activate aiphysicist  
pip install maturin  
pip install numpy  
### export rust module for python
maturin develop  
cargo check  
cargo test  
