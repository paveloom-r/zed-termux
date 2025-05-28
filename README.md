# Zed Termux extension

This extension adds support for [`bash-language-server`](https://github.com/bash-lsp/bash-language-server) and [`termux-language-server`](https://github.com/termux/termux-language-server) to aid in creation of some specific Bash scripts (e.g., Gentoo's [ebuilds](https://wiki.gentoo.org/wiki/Ebuild)).

> [!WARNING]
> Do note that this extension doesn't install the language servers, so their binaries are assumed to be found in at least one of the directories stored in the `PATH` environment variable.

Git mirrors:
- [Codeberg](https://codeberg.org/paveloom-r/zed-termux)
- [GitHub](https://github.com/paveloom-r/zed-termux)
- [GitLab](https://gitlab.com/paveloom-g/rust/zed-termux)

# Acknowledgements

The grammar files for Bash are adopted from the [grammar files](https://github.com/zed-industries/zed/tree/main/crates/languages/src/bash) used by Zed itself.
