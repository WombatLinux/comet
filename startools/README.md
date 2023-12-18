# startools
A set of tools for creating and managing stars (comet packages)

## Usage
### `startools`
```
startools 0.1.0
A set of tools for creating and managing stars (comet packages)

USAGE:
    startools [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information

SUBCOMMANDS:

    build     Build a star
    help      Prints this message or the help of the given subcommand(s)
    init      Initialize a star
    dependencies    Manage dependencies for a star
    check     Check a starfile for errors
```

### `startools init`
```
startools-init 0.1.0
Initialize a star

USAGE:
    startools init [FLAGS] [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information

OPTIONS:

ARGS:
    <NAME>    Name of the star to initialize
```

### `startools build`
```
startools-build 0.1.0
Build a star

USAGE:
    startools build [FLAGS] [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information

OPTIONS:

ARGS:
    <NAME>    Name of the star to build
```

### `startools check`
```
startools-check 0.1.0
Check a starfile for errors and warnings

USAGE:
    startools check [FLAGS] [OPTIONS] <STARFILE>

FLAGS:
    -h, --help       Prints help information

OPTIONS:

ARGS:
    <STARFILE>    Path to the starfile to check
```

### `startools new`
```
startools-new 0.1.0

A tool for creating new stars (comet packages)

USAGE:
    startools new [FLAGS] [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information

OPTIONS:

ARGS:
    <NAME>    Name of the star to create
```