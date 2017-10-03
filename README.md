PyCRust
=======

MWE of how to create a shared library from python code that can be dynamically
loaded and executed in rust via an intermediate layer in C. C/Python interface
currently only implements integer arguments and return values and would need to
be expanded to handle arbitrary types.

Setup 
=====

Install python headers: `sudo apt-get install python-dev` 

Install rust: `curl https://sh.rustup.rs -sSf | sh`

Set LD_LIBRARY_PATH and PYTHONPATH to search this repository, for example:
	1. `export LD_LIBRARY_PATH=./:$LD_LIBRARY_PATH`
	2. `export PYTHONPATH=$(pwd)`

Build the python shared library: `make`

Build rust: `cargo build`

Run rust: `cargo run`
