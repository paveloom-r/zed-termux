use zed_extension_api as zed;

struct Extension {}

impl zed::Extension for Extension {
    fn new() -> Self {
        return Extension {};
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let Some(language_server_path) = worktree.which(language_server_id.as_ref()) else {
            return Err(format!("couldn't find `{language_server_id}` in `PATH`",));
        };

        let mut args = vec![];
        match language_server_id.as_ref() {
            "bash-language-server" => {
                args.push(String::from("start"));
            }
            _ => {}
        }

        return Ok(zed::Command {
            command: language_server_path,
            args,
            env: vec![],
        });
    }
}

zed::register_extension!(Extension);
