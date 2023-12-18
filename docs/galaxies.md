# Galaxies
Galaxies are repositories of [stars](stars.md). A galaxy is usually a remote repository, such as a website, that contains
starfiles and a `repo.yml` file. The `repo.yml` file contains all the information about the contents of the galaxy, 
which is used by the comet package manager to install packages from the galaxy. 

## Example `repo.yml` file
```yaml
packages:
  comet:
    name: comet
    description: The package manager for Wombat Linux
    version: 1.0.0
    dependencies:
      system: 1.0.0
    authors: ["Wombat Linux", "afroraydude"]
    license: MIT
    checksum: insert-sha256-checksum-here
  startools:
    name: startools
    description: A utility for managing stars
    version: 1.0.0
    dependencies:
      system: 1.0.0
    authors: ["Wombat Linux", "afroraydude"]
    license: MIT
    checksum: insert-sha256-checksum-here
  system:
    name: system
    description: The base system for Wombat Linux
    version: 1.0.0
    dependencies: {}
    authors: []
    license: MIT
    checksum: insert-sha256-checksum-here
```

## Example directory structure
```
https://example.com/galaxy
├── /repo.yml
├── /startools.star
├── /system.star
└── /comet.star
```

## Managing galaxies
Managing galaxies requires a proper HTTP server. The HTTP server must be able to serve static files. The HTTP server must
also be able to serve the `repo.yml` file. All files related to the galaxy must be in the same directory. The directory
does not need to be in the root directory of the HTTP server, which is useful for hosting multiple galaxies on the same
server. Galaxies can be hosted on any HTTP server, such as Apache or Nginx, as long as the HTTP server can serve static
files.

If you choose to manage a galaxy you must be willing to maintain packages in the galaxy. This may require you to update
the `repo.yml` file, and update the checksums of the packages. You must also be willing to host the galaxy on a server
that is always online. If you are not willing to do this, you should not manage a galaxy. 

Testing software is a very important part of managing a galaxy. You must test that packages are properly installing and
uninstalling. You must also test that the `repo.yml` file is properly formatted. You must also test that the checksums
are correct. If you do not test your software, you may end up with a broken galaxy.

## The Official Galaxy
The official galaxy is hosted at https://galaxy.wombatlinux.org. For this, we use a simple S3 bucket. The S3 bucket is
configured to serve static files. The S3 bucket is also configured to serve the `repo.yml` file.


