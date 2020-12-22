# Voxport
[![Build Status](https://github.com/5mattmatt1/voxport/workflows/Rust/badge.svg)](https://github.com/5mattmatt1/voxport/actions?query=workflow%3ARust)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Simple tool for converting MagicaVoxel files

## Usage
USAGE:
    voxport [FLAGS] [OPTIONS] --stl --dae

FLAGS:
    -d, --dae        Exports in the Collada DAE format. Good for importing
    -h, --help       Prints help information
    -s, --stl        Exports in the STL (STereoLithography) format. Good for 3D Printing
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input MagicaVoxel file to convert
    -o, --output <output>    Output file of specified export format

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>