# LUKSmount

Utility to mount/unmount LUKS encrypted drives

## Table of Contents

- [LUKSmount](#luksmount)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [On Arch Linux](#on-arch-linux)
    - [On Other Distros](#on-other-distros)
      - [Using prebuilt binaries](#using-prebuilt-binaries)
      - [Using `cargo build`](#using-cargo-build)
      - [Using `cargo install`](#using-cargo-install)
  - [Usage](#usage)
  - [Changelog](#changlog)
  - [License](#license)

## Installation

### Prerequisites

- Linux
- Rust (optional, depends on installation method)
- Cryptsetup

### On Arch Linux

1. Add my arch repo using the following lines (Don't do this if you don't know what it does):

```conf
[sks-arch-repo]
SigLevel = Optional DatabaseOptional
Server = https://raw.githubusercontent.com/ShayKStage/$repo/main/$arch
```

2. Run `pacman -S luksmount`

### On Other Distros

#### Using prebuilt binaries

1. Go to the [latest release](https://github.com/ShayKStage/luksmount/releases/latest) and download `release.tar.gz`
2. Unpack it:

```bash
tar -xf release.tar.gz
```

3. Place the binaries in a directory on your path

#### Using `cargo build`

1. Clone the repository:

```bash
git clone https://github.com/ShayKStage/luksmount.git
cd luksmount
```

2. Build the Binaries:

```bash
cargo build --release --bins
```

3. Place the resultant binaries in a directory on your path

#### Using `cargo install`

1. Run `cargo install`

```bash
cargo install --git https://github.com/ShayKStage/luksmount.git --bins
```

You can add `--tag <TAG>` to install a specific release

## Usage

1. `luksmount`

```
Mount a LUKS encrypted filesystem

Usage: luksmount [OPTIONS] <DEV> <MNT> [FSTYPE]

Arguments:
  <DEV>     The block device to mount (eg. /dev/sdb1)
  <MNT>     The directory to use as the mountpoint (eg. /mnt)
  [FSTYPE]  The filesystem to mount it as (eg. ext4, btrfs, ntfs) [default: ext4]

Options:
  -m, --mkdir    Try to create the mount directory
  -h, --help     Print help
  -V, --version  Print version
```

2. `luksumount`

```
Unmount a LUKS encrypted filesystem

Usage: luksumount [OPTIONS] <MNT>

Arguments:
  <MNT>  The mount directory (eg. /mnt)

Options:
  -r, --rmdir    Try to remove the mount directory
  -h, --help     Print help
  -V, --version  Print version
```

## Changelog

[CHANGELOG.md](CHANGELOG.md)

## License

[MIT](https://choosealicense.com/licenses/mit/) [OR](https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/#d42-disjunctive-or-operator) [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
