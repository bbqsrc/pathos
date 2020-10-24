# pathos

[![Documentation](https://docs.rs/pathos/badge.svg)](https://docs.rs/pathos)
[![Actions Status](https://github.com/bbqsrc/pathos/workflows/CI/badge.svg)](https://github.com/bbqsrc/pathos/actions)

> _noun_ From Ancient Greek πάθος (páthos, “suffering”). The quality or property of anything which touches the feelings or excites emotions and passions, especially that which awakens tender emotions, such as pity, sorrow, and the like; contagious warmth of feeling, action, or expression; pathetic quality.

A natural API for finding OS-specific user or system directories, regardless of the operating system.

`pathos` specifically supports iOS and Android's idiosyncratic handling of 'user' directories.

## Features

### URL path handling

`pathos` supports Unicode-safe URL paths for `file:`, and a special custom `container:` scheme on iOS and Android. `pathos` can convert between ordinary `Path` types and these `Iri` types with ease, simplifying saving paths in a configuration system without worrying about `OsString` problems.

### Platform-agnostic modules

Don't care what platform you're building for and just want a project directory in the right place? `pathos::system` and `pathos::user` re-export the host platform's submodule.

### Platform-specific handling

Sometimes you have to care what platform you're on for special-cased weirdness. In those cases, the appropriate APIs are available on the relevant OS submodule.

### Full XDG support

XDG is the default mechanism for handling user directories on Linux, and can be opted into on other platforms by using the `pathos::xdg` module.

## Usage

Add the following to your Cargo.toml:

```toml
pathos = "0.2"
```

## Where is this used?

* [box](https://github.com/bbqsrc/box) - a modern replacement for the zip file format
* [pahkat](https://github.com/divvun/pahkat) - a cross-platform package management system

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
