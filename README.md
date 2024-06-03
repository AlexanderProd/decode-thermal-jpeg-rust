# Thermal JPEG Decoding Comparison: Python vs Rust

This repository contains two scripts, one in Python and one in Rust, that demonstrate the decoding the `exiftool_output.bin` file. The binary has been retrieved from the `exiftool` command line tool `exiftool -b -RawThermalImage`.

## Python Script

### How to run

```bash
python3 -m pip install -r requirements.txt
python3 main.py
```

The Python script uses the `decode` function from the `libjpeg` package to decode the binary file.

The script successfully prints the decoded jpeg file content and its shape (1024x768).

## Rust Script

### How to run

```bash
cargo run
```

The Rust script uses the `zune-jpeg` crate to attempt to decode the binary file.

While guessing the image format using the `guess_format` function from the `image` crate works, the `decode` function from the `zune-jpeg` crate fails to decode the binary file and returns:

```
Error decoding SOF Marker, Number of components cannot be zero.
```
