# libcomet
libcomet is a library made in Rust that provides a high-level interface to the [Comet package manager](../comet-cli/README.md).
It is also used by other Comet tools, such as the
[startools](../startools/README.md) program to create
`stars` (package files).

## Usage as a Program
The primary interface for interacting with the Comet package manager is through the `comet-cli` crate. This crate 
provides a command-line interface (CLI) for Comet, encapsulating its functionality in a user-friendly way. The 
`comet-cli` binary, typically installed as `comet`, is built using this crate.

## Usage as a Library
### In Rust Programs
For integration within Rust programs, the `comet` crate is available. It offers a high-level interface to the Comet 
package manager's functionality, allowing for seamless integration into Rust applications. For detailed usage 
instructions and examples, refer to the `comet` crate documentation.

### In Non-Rust Programs
To use Comet in non-Rust programs, a C-compatible library can be built using the command `cargo build --release --lib`
followed by using the `cbindgen` program to create the header file with 
`cbindgen --config cbindgen.toml --crate comet --output comet.h`. 
This command generates a `libcomet.so` file in the `target/release` directory and a `comet.h` file in the root 
directory.

On Wombat Linux, you can install the necessary components for external use, including the header file, with 
`comet install comet-external`. Link to the library in your C program with `-lcomet`.

Example of linking in a C program:
```c
#include <comet.h>

int main() {
    // Your code here
}
```

For more comprehensive guidelines on using libcomet in various programming environments, please refer to the dedicated 
documentation section for external usage.

## Contributing
If you would like to contribute to comet, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.
