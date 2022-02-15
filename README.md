# Eva
![Eva icon](data/eva.svg)
<br/>
Contents
========
* [Introduction](#introduction)
* [Features](#features)
* [Keybindings](#keybindings)
* [Building](#building)
## Introduction
Eva is a [gemini protocol](https://gemini.circumlunar.space/) browser written in
[Rust](https://rust-lang.org) using the [gtk+](https://gtk-rs.org/) toolkit. Eva
is currently alpha quality software under heavy development and as such is
missing features and may have bugs.
## Features
- [x] tabbed interface
- [ ] bookmarks
- [x] keyboard shortcuts
  - [ ] user configurable
- [x] user controlled styling
  - [x] fonts
  - [x] colors
- [x] back-forward list
- [ ] history

## Keybindings
| Key | Action |
| --- | --- |
| Ctrl/T | New tab |
| Ctrl/N | New window |
| Ctrl/W | Close tab |
| Ctrl/Q | Close window |
| Ctrl/R | Reload page |
| Alt/Home | Go to homepage |
| Alt/Left | Go back |
| Alt/Right | Go next |
| Ctrl/PageDown | Next tab |
| Ctrl/PageUp | Previous tab |
| Alt/[1-9] | nth tab |
| Ctrl/Shift/O | Open bookmarks |
| Ctrl/D | Bookmark page |
| Ctrl/H | Open History |
| Ctrl/Shift/P | Open preferences |
| Ctrl/Shift/A | Open about dialog |

## Building
```sh
# clone the source
git clone https://codeberg.org/jeang3nie/eva.git
cd eva
cargo run -- gemini://gemini.circumlunar.space
```
