# Nushell plugin to deal with SemVer versions

This is a plugin for the [`nu`](https://nushell.sh) shell to manipulate strings representing versions that conform to the [SemVer](https://semver.org) specification.

## Installation

You can compile from source by checking out this repository and running `cargo install --path .`, or installing the latest version with `cargo install nu_plugin_semver`.

In both cases you then need to register the plugin by running `register /path/to/nu_plugin_semver` from within `nu`. Typically, the plugin can be found in `$HOME/.cargo/bin/`.

## Examples
