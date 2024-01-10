# Nushell plugin to deal with SemVer versions

This is a plugin for the [`nu`](https://nushell.sh) shell to manipulate strings representing versions that conform to the [SemVer](https://semver.org) specification.

## Installation

You can compile from source by checking out this repository and running `cargo install --path .`, or installing the latest version with `cargo install nu_plugin_semver`.

In both cases you then need to register the plugin by running `register /path/to/nu_plugin_semver` from within `nu`. Typically, the plugin can be found in `$HOME/.cargo/bin/`.

## Examples
```nu
# Deconstruct version into record for manipulation, then back to string again.
❯ "1.2.3-alpha.1+build" | semver to-record
╭───────┬─────────╮
│ major │ 1       │
│ minor │ 2       │
│ patch │ 3       │
│ pre   │ alpha.1 │
│ build │ build   │
╰───────┴─────────╯

❯ "1.2.3-alpha.1+build" | semver to-record | update build "foo" | semver from-record
1.2.3-alpha.1+foo

# Or use the `bump` command:
❯ "1.2.3-alpha.1+build" | semver bump beta
1.2.3-beta.1+build

❯ "1.2.3-alpha.1+build" | semver bump major
2.0.0

# Sort versions using SemVer semantics:
❯ ["3.2.1", "2.3.4", "3.2.2", "2.3.4-beta.1", "2.3.4-alpha.1", "2.3.4-alpha.2"] | semver sort
╭───┬───────────────╮
│ 0 │ 2.3.4-alpha.1 │
│ 1 │ 2.3.4-alpha.2 │
│ 2 │ 2.3.4-beta.1  │
│ 3 │ 2.3.4         │
│ 4 │ 3.2.1         │
│ 5 │ 3.2.2         │
╰───┴───────────────╯

# Match a SemVer version against a version requirement.
>  "3.2.1" | semver match-req "3"
true

>  "3.2.1" | semver match-req ">=2"
true

>  "3.2.1" | semver match-req ">=2,<3"
false
```
