# RASCII
Simple CLI tool for creating colourful ASCII art!
It uses a kernel to compress and calculate the color and characters. 

Basically an attempt to port this Python module into Rust, using Rust structs instead of a purely functional approach:
https://github.com/GalOz666/PyAscii

*Using rust gives much faster results of course :)*

## Usage:

```
cargo run <image path> <Kernel Size [Default: 9]>
```
