# Eva
![Eva icon](data/eva.svg)
<br/>
Contents
========
* [Introduction](#introduction)
* [Building](#building)
* [Features](#features)
## Introduction
Eva is a [gemini protocol](https://gemini.circumlunar.space/) browser written in
[Rust](https://rust-lang.org) using the [gtk+](https://gtk-rs.org/) toolkit. Eva
is currently alpha quality software under heavy development and as such is
missing features and may have bugs.

## Building
```sh
# clone the source
git clone https://codeberg.org/jeang3nie/eva.git
cd eva
cargo run -- gemini://gemini.circumlunar.space
```
## Features
- [x] tabbed interface
- [ ] bookmarks
- [x] keyboard shortcuts
  - [ ] user configurable
- [ ] user controlled styling
  - [x] fonts
  - [ ] colors
- [x] back-forward list
- [ ] history
