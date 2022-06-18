# retrvid (rid)

`retrvid` (binary name: `rid`) is a command-line tool that stores and retrieves ids with a lookup name.

The ids are stored in a `toml` file in your default data directory. You can change the location of this file by setting the `RETRVID_DATA` environment variable. See the [Data Storage](#data-storage) section for more details.

`retrvid` is useful when you have a bunch of ids that you often need to copy, for instance, your student number, health number, etc. While primarly made for ids, you can store any text string with a simple lookup name and quickly copy it to your clipboard.

Note that the data is **NOT** stored securely, do not use this for any sensitive information on a shared computer.

## Usage

```txt
retrvid 0.1.0
Sujal Bolia <sujalbolia@gmail.com>

retrvid (rid) lets you store and retrieve ids with a lookup name

USAGE:
    rid [OPTIONS] [--] [NAME]

ARGS:
    <NAME>    The name of the id to get

OPTIONS:
    -p, --print              Print the id on the console
    -C, --no-copy            Do not copy the id to the system clipboard
    -l, --list               List all stored id names
    -a, --add <NAME> <ID>    Add provided name and id to the database
    -r, --remove <NAME>      Remove provided id (name) from the database
    -h, --help               Print help information
    -V, --version            Print version information
```

## Installation

You need [Rust][rust] to compile `retrvid`. Pre-compiled binaries are not available yet.

`cargo` is usually installed with Rust. If you don't have `cargo` installed, follow [the `cargo` installation documentation][cargo].

Once you have `cargo` installed, you can simply use `cargo install` or compile from source.

To use `cargo install`:

```sh
cargo install --git https://github.com/nextonesfaster/retrvid
```

`cargo` will install `retrvid` in its `bin` directory, which should already be in your `PATH`.

To compile from source:

```sh
# Clone this repository
$ git clone https://github.com/nextonesfaster/retrvid.git

# cd into the cloned repository
$ cd retrvid

# Compile using cargo with the release flag
$ cargo build --release
```

The executable will be at `./target/release/rid`. You can move it to your `PATH` to invoke `rid` from any directory.

## Data Storage

The ids are stored in a `toml` file. By default, the location of this file is `$DATA_DIR/retrvid/ids.toml` where `$DATA_DIR` is as follows:

| Platform |                `$DATA_DIR`                 |
| :------: | :----------------------------------------: |
|  Linux   |         `/home/Alice/.local/share`         |
|  macOS   | `/Users/Alice/Library/Application Support` |
| Windows  |      `C:\Users\Alice\AppData\Roaming`      |

You can override this by setting the `RETRVID_DATA` environment variable as the path of the data file. The environment variable takes precedence over the default location.

## License

`retrvid` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-MIT][mit] and [LICENSE-APACHE][apache] files for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[rust]: https://www.rust-lang.org/tools/install
[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
