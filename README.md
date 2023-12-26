# Comet Package Manager

Welcome to the repository for the Comet package manager [CLI](comet-cli/README.md) and associated tools. Comet is a 
next-generation package manager for Linux, designed with speed, ease of use, and stability in mind. Built on top of the 
[comet](comet/README.md) library, which is also written in Rust, Comet aims to provide a streamlined package management 
experience.

## Features
- Fast and efficient package management
- User-friendly CLI interface
- Robust library for advanced users and developers

## Components
This repository contains:
- `comet-cli`: The Comet package manager CLI
- `comet`: The Comet library for package management operations
- `startools`: Tools for creating and managing stars (Comet packages)

## Installation
Comet is installed on Wombat Linux by default. To install Comet on other Linux distributions, please download the
latest release from the [releases page](https://github.com/wombatlinux/comet/releases).

Although the CLI is created as `comet-cli`, it is recommended to create a symlink to `comet` or to rename it for ease of use:
```bash
# Option 1: Create a symlink
ln -s comet-cli comet

# Option 2: Rename the file
mv comet-cli comet
```

## Building from source
To build Comet from source, you will need to have the following installed:
- Rust (stable)
- Cargo
- Git
- C compiler (GCC or Clang)

To build Comet, run the following commands:
```bash
# Clone the repository
git clone https://github.com/wombatlinux/comet.git
cd comet

# Build Comet and all tools
cargo build --release --all-targets
```

## Usage
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
```

Basic usage of the `startools` command is as follows:
```bash
# Create a new star
startools new <package>

# Build a star
startools build <package>
```

## License
Comet is licensed under the [MIT License](LICENSE).

***Note: This package manager is currently in development and is not ready for general use.***
