# AI-Physicist

Discover new laws automatically from data.  
This project is still at a very early stage.  

## Preparation

Install Python (>= 3.10.0), conda (or pyenv)

Install Rust (https://www.rust-lang.org/tools/install)

Check and build the rust code.
```
cargo check
cargo build
```


## Installation

### use `maturin`

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

### use `ai_physicist.so`

First, run
```
cargo build --release
```
to produce a `libai_physicist.so` file at `{work folder}/target/release/`.
Then, change its name and copy the file to `{work folder}/ai_physicist.so`.
```
cp target/release/libai_physicist.so python/ai_physicist.so
```
Remember to create a virtual environment with all dependencies in `requirements.txt` are installed:
```
conda activate aiphy
pip install -r requirements.txt
```
Finally, Testing AI-physicist
```
python python/test9.py
```
 
## QA
1.
Q:
```
No usable m4 in $PATH or /usr/5bin (see config.log for reasons).
```
A:
```
sudo apt install m4
```
2.
```
The system library `openssl` required by crate `openssl-sys` was not found.
```
A:
```
sudo apt install libssl-dev
sudo apt install openssl
```
3.
```
not found CC
not found FC
not found HOSTCC
...
called `Result::unwrap()` on an `Err` value: FortranCompilerNotFound
```
A:
```
sudo apt install gfortran
```