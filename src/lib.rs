use zed_extension_api::{self as zed, settings::LspSettings};

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

        if let Ok(lsp_settings) = zed::settings::LspSettings::for_worktree("fortls", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    let args = binary
                        .arguments
                        .unwrap_or(vec!["--lowercase_intrinsics".to_string()]);
                    return Ok(zed::Command {
                        command: path,
                        args,
                        env: Default::default(),
                    });
                }
            }
        }

        Ok(zed::Command {
            command: path,
            args: vec!["--lowercase_intrinsics".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(_language_server_id.as_ref(), _worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(_language_server_id.as_ref(), _worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(FortranExtension);
