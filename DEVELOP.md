# Developing guide

## Developing the extension locally

1. Clone this repository
2. Install the `wasm32-wasip2` target via `rustup target add wasm32-wasip2`
3. Start Zed in the foreground via `zed --foreground`
4. Go to the Extensions tab and press Install Dev Extension
5. Select the directory where the repository resides
6. Whenever a change is made, press Rebuild next to the extension

See also [Developing an Extension Locally][1] in the official documentation.

[1]: https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally

## Updating the extension

1. Clone the [extensions repository][2]
2. Update the extension
    1. Update the extension's version in `extensions.toml`
    2. Update the Git submodule
        1. `git submodule init extensions/termux`
        2. `git submodule update --remote extensions/termux`
    3. Stage the changes or make a commit (necessary for step 5)
3. Download the `zed-extension` binary used by the upstream in CI
    1. Get the current hash stored in `.github/workflows/ci.yml`
        - `HASH=$(grep -oP "(?<=ZED_EXTENSION_CLI_SHA: )\w+" .github/workflows/ci.yml)`
    2. Fetch the binary from the object storage
        - `wget "https://zed-extension-cli.nyc3.digitaloceanspaces.com/$HASH/x86_64-unknown-linux-gnu/zed-extension"`
    3. Make it executable with `chmod +x zed-extension`
4. Run `pnpm install` to install dependencies
5. Run `pnpm package-extensions` to check whether the extension can be packaged
6. If all good, create a pull request to the upstream

[2]: https://github.com/zed-industries/extensions

See also [Updating an extension][3] in the official documentation.

[3]: https://zed.dev/docs/extensions/developing-extensions#updating-an-extension

> [!note]
> Alternatively, the `zed-extension` binary can be compiled from the
> [`extensions_cli` crate][4] in the [`zed` repository][5].

[4]: https://github.com/zed-industries/zed/tree/main/crates/extension_cli
[5]: https://github.com/zed-industries/zed
