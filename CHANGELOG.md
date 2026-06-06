# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog][common-changelog],
and this project adheres to [Semantic Versioning][semantic-versioning].

[common-changelog]: https://common-changelog.org
[semantic-versioning]: https://semver.org/spec/v2.0.0.html

## 0.1.2 - 2025-06-06

### Fixed

- Ship the Bash grammar again (until `zed-extension` CLI tool can handle the
  built-in grammars) (`5a0ef1d`)

## 0.1.1 - 2025-05-30

### Fixed

- Don't hard code the Bash grammar (rely on the one shipped with Zed itself)
  (`e284cb2`)

## 0.1.0 - 2025-05-29

_Initial release._

### Added

- Add support for [`bash-language-server`][bash-language-server] and
  [`termux-language-server`][termux-language-server] to aid in editing
  [ebuilds][ebuild] (`78cbaf9`)

[ebuild]: https://wiki.gentoo.org/wiki/Ebuild
[bash-language-server]: https://github.com/bash-lsp/bash-language-server
[termux-language-server]: https://github.com/termux/termux-language-server
