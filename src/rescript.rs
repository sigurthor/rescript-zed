use std::{borrow::Cow, env, fs};
use zed_extension_api::serde_json::Value;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/@rescript/language-server/out/cli.js";
const PACKAGE_NAME: &str = "@rescript/language-server";

struct ReScriptExtension {
    did_find_server: bool,
}

#[derive(Debug, Default)]
struct Settings {
    initialization_options: Option<Value>,
    version: Option<String>,
}

fn parse_version(settings_value: &Value) -> Option<String> {
    let settings = settings_value.as_object()?;

    settings
        .get("zed")
        .and_then(|value| value.as_object())
        .and_then(|zed| {
            zed.get("version")
                .or_else(|| zed.get("serverVersion"))
                .and_then(|value| value.as_str())
        })
        .map(|value| value.to_string())
        .or_else(|| {
            settings
                .get("version")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string())
        })
}

fn parse_settings(lsp_settings: LspSettings) -> Settings {
    Settings {
        initialization_options: lsp_settings.initialization_options,
        version: lsp_settings.settings.as_ref().and_then(parse_version),
    }
}

fn merge_json(target: &mut Value, source: Value) {
    match (target, source) {
        (Value::Object(target), Value::Object(source)) => {
            for (key, value) in source {
                if let Some(target_value) = target.get_mut(&key) {
                    merge_json(target_value, value);
                } else {
                    target.insert(key, value);
                }
            }
        }
        (target, source) => *target = source,
    }
}

fn default_initialization_options() -> Value {
    zed::serde_json::json!({
        "extensionConfiguration": {
            "askToStartBuild": true,
            "logLevel": "info",
            "inlayHints": {
                "enable": false,
                "maxLength": 25
            },
            "codeLens": false,
            "binaryPath": null,
            "platformPath": null,
            "runtimePath": null,
            "signatureHelp": {
                "enabled": true,
                "forConstructorPayloads": true
            },
            "incrementalTypechecking": {
                "enable": true,
                "acrossFiles": false
            },
            "cache": {
                "projectConfig": {
                    "enable": true
                }
            },
        }
    })
}

impl ReScriptExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn get_lsp_settings_for_worktree(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Settings> {
        let settings = zed::settings::LspSettings::for_worktree(server_id.as_ref(), worktree);
        match settings {
            Err(_) => Ok(Settings::default()),
            Ok(lsp_settings) => Ok(parse_settings(lsp_settings)),
        }
    }

    fn initialization_options_for_worktree(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Value> {
        let settings = self.get_lsp_settings_for_worktree(server_id, worktree)?;
        let mut initialization_options = default_initialization_options();

        if let Some(user_initialization_options) = settings.initialization_options {
            merge_json(&mut initialization_options, user_initialization_options);
        }

        Ok(initialization_options)
    }

    fn server_script_path(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Cow<'static, str>> {
        let server_exists = self.server_exists();

        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.into());
        }

        zed::set_language_server_installation_status(
            server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let settings = self.get_lsp_settings_for_worktree(server_id, worktree)?;

        let version = if let Some(user_version) = settings.version {
            user_version
        } else {
            zed::npm_package_latest_version(PACKAGE_NAME)?
        };

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);

            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{}' did not contain expected path '{}'",
                            PACKAGE_NAME, SERVER_PATH
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;

        Ok(SERVER_PATH.into())
    }
}

impl zed::Extension for ReScriptExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(server_id, worktree)?;

        let current_dir =
            env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                current_dir
                    .join(server_path.as_ref())
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(self.initialization_options_for_worktree(
            server_id, worktree,
        )?))
    }
}

zed::register_extension!(ReScriptExtension);
