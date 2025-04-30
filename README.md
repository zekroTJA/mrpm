# mrpm

A Modrinth plugin/package manager

## Usage

```
$ mrpm --help
A Modrinth plugin/package manager

Usage: mrpm [OPTIONS] <COMMAND>

Commands:
  init     Initialize a new project
  install  Install packages
  update   Install packages
  search   Search for packages
  help     Print this message or the help of the given subcommand(s)

Options:
  -D, --directory <DIRECTORY>  Target project directory [default: .]
  -h, --help                   Print help
  -V, --version                Print version
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekrotja/mrpm/releases) or you can install it using cargo install.

```
cargo install --git https://github.com/zekrotja/mrpm
```

Also, you can simply use the provided install script:

```bash
curl -sSfL https://raw.githubusercontent.com/zekroTJA/mrpm/refs/heads/main/scripts/install.sh | sudo bash -
```