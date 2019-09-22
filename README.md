# RASCII
Simple CLI tool for creating colourful ASCII art!
It uses a kernel to compress and calculate the color and characters. 

Basically an attempt to port this Python module into Rust, using Rust structs instead of a purely functional approach:
https://github.com/GalOz666/PyAscii

Albiet a slower version (something to do with printing in rust) and with less image processing - it was a cool experiment. 

## Usage:

```
cargo run <image path> <Kernel Size [Default: 9]>
```
