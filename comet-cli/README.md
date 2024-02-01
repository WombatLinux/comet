# Comet Package Manager
Comet is a next-generation package manager for Linux. It is designed to be fast, easy to use, and stable.
It is built on top of the [comet](../comet/README.md) library, which is written in Rust as well.

## Installation
Comet is the offical package manager for [Wombat Linux](http://wombatlinux.org), and is installed by default. If you are
not using Wombat Linux, you can install Comet by downloading the latest release from the 
[releases page](https://github.com/wombatlinux/comet/releases), or by building it from source. To build from source, 
follow the "Building from source" section below.


## Building from source

### Prerequisites
To build Comet from source, you will need the following:
- Rust 1.51 or newer
- Cargo 1.51 or newer
- Git
- A C compiler (GCC, Clang, etc.)
- Your OS's development tools (`build-essential` on Debian/Ubuntu, `base-devel` on Arch, etc.) for building comet
- `libssl`

### Building
To build Comet from source, run the following commands:
```bash
git clone https://github.com/wombatlinux/comet.git
cd comet
cargo build --release --all-targets
```

### Installing
Installing after building is as simple as copying the binary to a directory in your PATH. For example:
```bash
sudo cp target/release/comet-cli /usr/local/bin
sudo ln -s /usr/local/bin/comet-cli /usr/local/bin/comet
```

The binary will be located at `target/release/comet`.

## Usage
Comet is designed to be easy to use. It has a simple command-line interface, and is designed to be easy to use.
To see the usage information, run `comet --help`.

Basic usage of Comet is as follows:
```bash
# Install a star
comet install <package>

# Update a star
comet update <package>

# Remove a star
comet remove <package>

# Update the package database
comet update-cache

# Update all stars
comet update-all

# List all installed stars
comet list

# List available stars
comet list-available
```
