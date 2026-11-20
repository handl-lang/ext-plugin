# ext-plugin

`hnl-ext-plugin` — `hnl plugin` as a dynamic `hnl` extension.

An `ext-*` crate: plugin management packaged as a
`libhnl_ext_plugin.{so,dylib,dll}` shared library for the dumb `hnl` host. The
host discovers it via `--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev
`target/debug/` next to `hnl`.

## Commands

```text
hnl plugin <add|remove|info> [args...]
```

Runs the `plugin` capability of `handl-plugin-ext-plugin`'s
`PluginManagerPlugin` inside a `DefaultHost`. Object responses
(`{"added": …}`) print nothing; errors go to stderr with exit 2.
`plugin add <name> --path …` still requires a `load` capability plugin, which
this extension does not register.

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. The crate builds a `cdylib` +
`rlib`; `cargo test` links the `rlib` statically.

```sh
cargo build        # target/debug/libhnl_ext_plugin.so
HANDL_EXT_DIR=$PWD/target/debug ../host-cli/target/debug/hnl plugin info
```

Sibling-repo deps resolve from sibling checkouts via the `[patch]` tables;
without them the `branch = "main"` git sources are used.

## License

MIT.
