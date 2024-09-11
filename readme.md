# AI-Physicist

Discover new laws automatically from data.  
This project is still at a very early stage.  

## Preparation

Install Python (>= 3.10.0), conda (or pyenv)

Install Rust (https://www.rust-lang.org/tools/install)

Remember to add the path `{path/to/rust}/bin` to your environment variables.
And remember to install `maple2024` at `/opt/maple2024`.

Check and build the rust code.
```
cargo check
cargo build
```


## Installation


### create `ai_physicist.so` and use it

First, run
```
cargo build --release
```
to produce a `libai_physicist.so` file at `{work folder}/target/release/`.
Then, change its name and copy the file to `{work folder}/ai_physicist.so`.
```
cp target/release/libai_physicist.so python/ai_physicist.so
```

Now you can use `pyenv, conda` etc. to set up a working environment of python. Take conda as an example, first create a virtual environment as follows:
```
conda create -n aiphysicist python=3.10 
```

Then, activate the virtual environment.
```
conda activate aiphysicist
```

Install all dependencies in `requirements.txt`:
```
pip install -r requirements.txt
```
Finally, Testing AI-physicist
```
python python/test9.py
```
 
### use `maturin`

Work in a virtual environment. First, activate the environment. Then, install `maturin`, and run `maturin develop` to build wheel for `ai_physicist`.

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


## QA (when running `cargo build`, these errors may occur)
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