# mrpm

A Modrinth plugin/package manager

## Usage

```
$ mrpm --help
A Modrinth plugin/package manager

Usage: mrpm [OPTIONS] <COMMAND>

Commands:
  init     Initialize a new project [aliases: new]
  install  Install packages [aliases: i, add, a]
  update   Update packages [aliases: u]
  search   Search for packages [aliases: s]
  remove   Remove packages [aliases: r]
  help     Print this message or the help of the given subcommand(s)

Options:
  -D, --directory <DIRECTORY>  Target project directory [default: .]
  -v, --verbose                Enable verbose output
  -h, --help                   Print help
  -V, --version                Print version
```

### Workflow

First of all, you need to initialize a new project. You can either use the setup wizard by passing no initialization parameters, or pass everything via parameters.

```bash
# use wizard
mrpm init

# pass via parameters
mrpm init \
    --loader paper \
    --version 1.21.4 \
    --artifacts-dir plugins \
    --minimum-version-type beta

# initialize in another directory
mrpm --directory /home/servers/myserver init
```

After that, you can install plugins or mods via the `install` command.

```bash
mrpm install pl3xmap worldedit

# You can also specify specific versions to Install
mrpm install pl3xmap@1.21.4-525
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

```

```
