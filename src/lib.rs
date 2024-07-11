use zed_extension_api as zed;

struct FortranExtension {}

impl zed::Extension for FortranExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("fortls")
            .ok_or_else(|| "Fortls not installed.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["--lowercase_intrinsics".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(FortranExtension);
