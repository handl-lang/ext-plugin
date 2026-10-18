//! `hnl-ext-plugin` — the `plugin` extension for `hnl` (ADR-0149).
//!
//! The plugin-management surface (`plugin add|remove|info`) is the
//! `plugin` capability of `handl-plugin-ext-plugin`'s
//! `PluginManagerPlugin` — the same destination the built-in host's
//! `cli`-capability fallback routed unknown commands to. This crate is the
//! *packaging*: it exports the ADR-0149 ABI (`handl_ext_manifest` /
//! `handl_ext_dispatch` / `handl_ext_free`) and runs the capability inside
//! a `DefaultHost`, matching the pre-dumb-host behavior — object results
//! (`{"added": …}`) printed nothing, errors went to stderr with exit 2.

use handl_ext::{ExtRequest, ExtResponse, ExtensionManifest};
use handl_platform::{CapabilityRequest, DefaultHost};

handl_ext::export!(
    ExtensionManifest::new("plugin")
        .version(env!("CARGO_PKG_VERSION"))
        .command("plugin", "plugin <add|remove|info> [args...]"),
    dispatch
);

/// ABI dispatch: only the `plugin` command is declared.
fn dispatch(cmd: &str, req: &ExtRequest) -> ExtResponse {
    match cmd {
        "plugin" => run(&req.args),
        other => ExtResponse::error(2, format!("plugin: unknown command `{other}`\n")),
    }
}

/// Route `plugin` args through the `plugin` capability (payload `{args}`;
/// `PluginManagerPlugin` reads `args[0]` as the action). Same rendering as
/// the old `cli_dispatch`: a string response prints to stdout; object
/// results produce no output; errors go to stderr with exit 2.
fn run(args: &[String]) -> ExtResponse {
    let mut stdout = String::new();
    let mut stderr = String::new();

    let mut host = DefaultHost::new();
    host.register_plugin(handl_plugin_ext_plugin::PluginManagerPlugin::new());
    if let Err(e) = host.load() {
        stderr.push_str(&format!("composition error: {e}\n"));
        return ExtResponse {
            code: 2,
            stdout,
            stderr,
        };
    }
    if let Err(e) = host.resolve_composition() {
        stderr.push_str(&format!("composition error: {e}\n"));
        return ExtResponse {
            code: 2,
            stdout,
            stderr,
        };
    }

    let resp = host.run(&CapabilityRequest {
        capability: "plugin".into(),
        payload: serde_json::json!({ "args": args }),
    });

    match resp {
        Ok(r) => {
            if let Some(s) = r.0.as_str() {
                stdout.push_str(&format!("{s}\n"));
            }
            ExtResponse {
                code: 0,
                stdout,
                stderr,
            }
        }
        Err(e) => {
            stderr.push_str(&format!("plugin: {e}\n"));
            ExtResponse {
                code: 2,
                stdout,
                stderr,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn plugin_info_runs_the_capability() {
        let res = run(&["info".into()]);
        assert_eq!(res.code, 0, "stderr: {}", res.stderr);
    }

    #[test]
    fn plugin_missing_action_is_an_error() {
        let res = run(&[]);
        assert_eq!(res.code, 2);
    }

    /// The `export!`-generated manifest symbol, called directly.
    #[test]
    fn manifest_declares_plugin() {
        let raw = handl_ext_manifest();
        assert!(!raw.is_null());
        let json = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        unsafe { handl_ext_free(raw) };
        let manifest: ExtensionManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.name, "plugin");
        let names: Vec<&str> = manifest.commands.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["plugin"]);
    }
}
