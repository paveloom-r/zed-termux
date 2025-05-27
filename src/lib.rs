use zed_extension_api as zed;

struct Extension {}

impl zed::Extension for Extension {
    fn new() -> Self {
        return Extension {};
    }
}

zed::register_extension!(Extension);
