# Stars: Package Files in Comet

Stars are the fundamental package files used by the Comet package manager. They are `tarballs` – compressed archives – that encapsulate everything needed to install, update, and remove a package. This format ensures efficient distribution and integrity of package content.

Each Star contains the following components:
- `info.yml`: A YAML file containing metadata about the package, such as the version, dependencies, and author information.
- `install`: A script with commands executed during the package installation.
- `remove`: A script with commands executed during the package removal.
- `package`: A directory containing the actual files to be copied to the system upon installation.

The structure of a Star is as follows:
```
package_name.star
├── info.yml
├── install
├── remove
└── package
    ├── file1
    ├── file2
    ├── file3
    ├── directory1
    │   ├── file1
    │   ├── file2
    │   └── ...
    └── ...
```

## Creating a star
The easiest way to create a star is to use the `startools` utility. This utility will create a star for you. To use this 
utility, run the following command:
```bash
# Initialize an empty star
startools init <package>

# Use the interactive mode
startools new <package>
```

## Publishing a star
To publish a star, you must first create a star. Then, you must build the star file. To build the star file, run the
`startools build` command. This command takes one argument, the name of the star to build. This command will create a
`starfile`. This file is a `tarball` that contains the star. To publish the star, you must upload the `starfile` to a 
[Galaxy](galaxies.md), and update the `repo.yml` file of the Galaxy. The `repo.yml` file is a file that contains the 
list of available stars in the Galaxy. 

Command for building a star:
```bash
# Build a star
startools build <package>
```

The output will usually contain the information to put into the `repo.yml` file. The output will look like this:
```yaml
# Output of the build command
name: test
description: A test package
version: 1.0.0
stars:
  system: 1.0.0
  comet: 1.0.0
authors:
- Wombat Linux
- afroraydude
license: MIT
checksum: 81c2fba96744b22f62c24b668aa8d88012e290e17370d042e8e3fb87173e6599
````

This can be added to the `repo.yml` file like this:
```yaml
packages:
  # other packages go here
  test:
    name: test
    description: A test package
    version: 1.0.0
    stars:
      system: 1.0.0
      comet: 1.0.0
    authors:
      - Wombat Linux
      - afroraydude
    license: MIT
    checksum: 81c2fba96744b22f62c24b668aa8d88012e290e17370d042e8e3fb87173e6599
```

## Constellations

Constellations in Comet are akin to meta-packages found in other package managers. They serve as a convenient way to 
group and manage related stars (packages) that are typically installed together, streamlining the installation process 
for users. This is especially useful for complex setups like desktop environments or development toolsets.

A constellation is managed through a starfile, which primarily lists dependencies on other stars. Additionally, 
constellations can include specific `install` and `remove` scripts for customized setup or cleanup tasks.

### The `system` Constellation
A prime example of a constellation is the `system` package, which forms the base system of Wombat Linux. This 
constellation is unique in that it doesn't contain additional data in its `install` or `remove` scripts. Instead, it 
focuses on dependencies critical to the operating system, such as `libc`, `kernel`, and `libssl`. This design allows 
the entire core system to be installed or updated with simple commands like `comet install system` or 
`comet update system`, respectively.

This approach not only simplifies the installation and update processes but also ensures that users can easily maintain 
and customize their system with minimal effort.

