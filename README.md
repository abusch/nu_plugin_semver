# Nushell plugin to deal with SemVer versions

This is a plugin for the [`nu`](https://nushell.sh) shell to manipulate strings representing versions that conform to the [SemVer](https://semver.org) specification.

## Installation

You can compile from source by checking out this repository and running `cargo install --path .`, or installing the latest version with `cargo install nu_plugin_semver`.

In both cases you then need to register the plugin by running `plugin add /path/to/nu_plugin_semver` from within `nu`. Typically, the plugin can be found in `$HOME/.cargo/bin/`.

## Examples
```nu
# Parse a semver string into a semver value
> let v = "1.2.3-alpha.1+build" | into semver

# You can access individual fields of a version:
> $v.minor
2

# You can bump a version to different levels:
> $v | semver bump patch
1.2.3+build

> $v | semver bump major
2.0.0

# Semver values can be turned back into strings using
> $v | into string

# Semver values can be matched against a version requirement:
> let v = "3.2.1" | into semver

>  $v | semver match-req "3"
true

>  $v | semver match-req ">=2"
true

>  $v | semver match-req ">=2,<3"
false

# Semver values can be sorted, according to semver semantics
❯ ["3.2.1", "2.3.4", "3.2.2", "2.3.4-beta.1", "2.3.4-alpha.1", "2.3.4-alpha.2"] | into semver | sort
╭───┬───────────────╮
│ 0 │ 2.3.4-alpha.1 │
│ 1 │ 2.3.4-alpha.2 │
│ 2 │ 2.3.4-beta.1  │
│ 3 │ 2.3.4         │
│ 4 │ 3.2.1         │
│ 5 │ 3.2.2         │
╰───┴───────────────╯

```
