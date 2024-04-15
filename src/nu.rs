use zed_extension_api::{self as zed, Result};

struct NuExtension;

impl zed::Extension for NuExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("nu")
            .ok_or_else(|| "nu is not installed".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["--lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NuExtension);