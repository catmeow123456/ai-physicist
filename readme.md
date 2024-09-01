# AI-Physicist

Discover new laws automatically from data.  
This project is still at a very early stage.  

## Installation

You can use `pyenv, conda` etc. to set up environment. Take pyenv as an example, first create a virtual environment as follows:
```
pyenv virtualenv 3.10.0 aiphysicist  
```

Then, activate the virtual environment.
```
pyenv activate aiphysicist
```

Install `maturin`, and run `maturin develop` to build wheel for `ai_physicist`.

```
pip install maturin
maturin develop
```

Now you can work in python, install all dependencies.

```
pip install -r requirements.txt
```
Testing AI-physicist
```
python python/test9.py
```
