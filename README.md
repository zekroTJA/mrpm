# mrpm

A Modrinth plugin/package manager

## Usage

```
$ mrpm --help
< TODO: insert help here >
```


## Configuration

Configuration can be provided as `TOML`, `YML` or `JSON` file either in the current working directory as `fw.*` or in the users home config directory, which maps to the following directories depending on the OS.
- Linux: `$HOME/.config/fw/config.*`
- macOS: `$HOME/Library/Application Support/de.zekro.fw/config.*`
- Windows: `%APPDATA%/zekro/fw/config.*`

You can also pass a configuration via the `--config` parameter.

Following, you can see an example configuration in `TOML` format.

```toml
< TODO: insert config example here >
```


## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekrotja/mrpm/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/zekrotja/mrpm
```