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

## Installation

### Prerequisites

- Linux
- Rust (optional, depends on installation method)
- Cryptsetup

### On Arch Linux

1. If you don't already have my arch repo in your pacman.conf, add it using the following lines:

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

