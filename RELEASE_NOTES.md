Contents
========
* [unreleased](#unreleased)
* [0.4.1](#0.4.1-release)
* [0.4.0](#0.4.0-release)
* [0.3.1](#0.3.1-release)
* [0.3.0](#0.3.0-release)

## Unreleased
* Add Kennedy search engine to default search engines
* Add Gemipedia to default search engines
* Move tab sub-structures to ui definition files and subclass as Gobjects

## 0.4.1 release
* Fixes a regression caused by the new parser. This was actually due to a
workaround for the old parser, which was inserting an empty line at the end of
every preformatted block. The workaround truncated the final character from the
block, removing the newline, but this was no longer required with the new
parser and was instead cutting off the final non-whitespace character.

## 0.4.0 release
* Gemini - Use colored icons to differentiate link types
* Gemini - Handle sensitive input requests
* Gopher - handle http[s] links
* Spartan - initial support (sans upload support)
* Spartan - support Spartan uploads
* Rewrite of gemtext parser in GemView
* Major refactor of actions handing, removing 293 total lines of code

## 0.3.1 release
* Fix issue with the wrong color being read during preferences update
* Adjust default theme

## 0.3.0 release
* Handle downloads
* Use `cargo xtask` pattern for distribution
* Simplify imports
* Save pages
