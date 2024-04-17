use zed_extension_api::{self as zed, Result};

struct CapnpExtension;

impl zed::Extension for CapnpExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("Not implemented".into())
    }
}

zed::register_extension!(CapnpExtension);
