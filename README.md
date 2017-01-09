# Volatile
This crate is defines a wrapper interface for Volatile values and references with the ```VolatileCell``` data type. It is a wrapper on the underlying Rust functions ```write_volatile``` and ```read_volatile``` from ```core::ptr```.
The purpose of ```VolatileCell``` is to represent hardware registers and memory-mapped I/O.

### Dependencies
This package has no dependencies other than a recent ```stable``` version of ```rustc```. It can be used without ```std```.

