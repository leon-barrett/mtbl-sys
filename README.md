# mtbl Rust Bindings

This library provides Rust FFI interface to the
[mtbl](https://github.com/farsightsec/mtbl) C library for dealing with
SSTables (write-once sorted map files).

SSTables are basically constant on-disk maps, like those used by
[CDB](http://www.corpit.ru/mjt/tinycdb.html) (which also has [Rust
bindings](https://github.com/andrew-d/tinycdb-rs), except using sorted maps
instead of hashmaps. For more information, see the [mtbl
README](https://github.com/farsightsec/mtbl).

Version 0.1.X of mtbl-sys covers the 0.6 version of the MTBL C library.

## Dependencies

In order to use the `mtbl-sys` crate, you must have a Unix system with the
`libmtbl` library installed where it can be found by `pkg-config`.

On Debian-based Linux distributions, install the `libmtbl-dev` package:

```
sudo apt-get install libmtbl-dev
```

## Usage

Add `mtbl-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
mtbl-sys = "0.1.0"
```

Import the `mtbl_sys` crate and use the functions as they're defined in the
native `libmtbl` library. See the `libmtbl` API documention man pages for
more usage information.

```rust
extern crate mtbl_sys as mtbl;
```

## Function documentation

For documentation about each function, see MTBL's extensive man pages, e.g.
`man mtbl_reader`.

## License

Copyright 2016 Leon Barrett

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
