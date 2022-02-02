# Eva
Contents
========
* [Introduction](#introduction)
* [Building](#building)
* [Features](#features)
## Introduction
Eva is a [gemini protocol](https://gemini.circumlunar.space/) browser written in
[Rust](https://rust-lang.org) using the [gtk+](https://gtk-rs.org/) toolkit.

## Building
This project is currently just a wip shell. The browser widget is implemented as
a separate Rust *crate* which you will need a local copy of.
```sh
# clone the source
git clone https://codeberg.org/jeang3nie/eva.git
# clone the browser widget crate
git clone https://codeberg.org/jeang3nie/gemview.git
cd eva
cargo run -- gemini://gemini.circumlunar.space
```
## Features
- [x] tabbed interface
- [ ] bookmarks
- [x] keyboard shortcuts
  - [ ] user configurable
- [ ] user controlled styling
- [ ] history
