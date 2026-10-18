# ext-plugin — `hnl plugin` as a dynamic `hnl` extension

Plugin management packaged as a `libhnl_ext_plugin.{so,dylib,dll}` shared
library for the dumb `hnl` host (ADR-0149). The host discovers it via
`--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev `target/debug/` next
to `hnl`.

## Commands

```text
hnl plugin <add|remove|info> [args...]
```

Runs the `plugin` capability of `handl-plugin-ext-plugin`'s
`PluginManagerPlugin` inside a `DefaultHost` — the same destination the old
host's `cli` fallback routed unknown commands to. As before, object
responses (`{"added": …}`) print nothing; errors go to stderr with exit 2.
`plugin add <name> --path …` still requires a `load` capability plugin,
which this extension does not register (matching the old fallback, which
also did not).

## Dev

```sh
cargo build        # target/debug/libhnl_ext_plugin.so
cargo test         # unit tests link the rlib statically
HANDL_EXT_DIR=$PWD/target/debug ../host-cli/target/debug/hnl plugin info
```

Sibling-repo deps resolve from sibling checkouts via the `[patch]` tables
(ADR-0128 §Resolution); without them the `branch = "main"` git sources are used.
