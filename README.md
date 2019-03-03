# steno-lookup

Tool to look up the stroke for a word.

Build Status:

* Debian: [![builds.sr.ht Debian status](https://builds.sr.ht/~wezm/steno-lookup/debian.yml.svg)](https://builds.sr.ht/~wezm/steno-lookup/debian.yml?)
* FreeBSD: [![builds.sr.ht FreeBSD status](https://builds.sr.ht/~wezm/steno-lookup/freebsd.yml.svg)](https://builds.sr.ht/~wezm/steno-lookup/freebsd.yml?)

[Issue Tracker](https://todo.sr.ht/~wezm/steno-lookup)

<img src="https://git.sr.ht/%7Ewezm/steno-lookup/blob/master/screenshot.png" alt="Screenshot of steno-lookup in a terminal window" width="466" />

## Compatibility

`steno-lookup` has been tested on:

* Linux
* macOS
* Windows

## Installing

### From Binary Release

[Latest Release][release]

`steno-lookup` is a single small binary. To download the latest release do the following:

    curl -L https://releases.wezm.net/steno-lookup/steno-lookup-v0.3.0-arm-unknown-linux-gnueabihf.tar.gz | tar zxf -

The binary should be in your current directory and can be run as follows:

    ./steno-lookup

Feel free to move it elsewhere (`~/.local/bin` for example).

### From Source

**Note:** You will need the [Rust compiler installed][rust].

    git clone https://git.sr.ht/~wezm/steno-lookup
    cargo install --path steno-lookup

## License

This project is dual licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) **or**
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
