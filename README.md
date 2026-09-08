# A Crook plugin

The shortest one there is: a word in the header, asking for no permission at all. Press **Use
this template**, and the loop below is the whole of what writing a plugin looks like.

```sh
cargo build --release --target wasm32-unknown-unknown
crook --dev-plugin .
```

That runs the module straight out of `target/`, in the window you already have open, and runs
it again every time you build it. Nothing is installed and nothing is left on the machine.

## What is here

```
crates/plugin/src/lib.rs        five exports, a manifest, and what it draws
crates/plugin/src/sys.rs        the two host functions it uses, stubbed off wasm
crates/plugin/src/lib_tests.rs  what it draws, tested with no terminal in sight
```

Three hundred lines, most of them comments. The parts to change first:

- **`manifest()`** — the id is `owner/name` and the owner is your GitHub account. It is what a
  permission, a keybinding and a registry entry all name, so change it before you publish.
- **`SLOT`** — where it draws. `header.right` is the end of the title bar; `tab.row.mark` is the
  mark on every tab; `pane.chips` is the row under what you are typing; `block.menu` is the menu
  on a finished command.
- **`tree()`** — what it draws, as a `Node`. There is no colour and no pixel in it: a `Node` says
  what a thing *is* and the terminal draws it in whatever theme is in force, which is what makes
  a plugin written today come out right in a theme written years from now.

## Asking for something

This asks for nothing, and a plugin that asks for nothing draws from the moment it is installed.
Everything else is a `Capability` in the manifest — one host, one path, one command, never a
category — and a person answers for it on the Plugins page before your plugin reaches any of it.
Add one and their card will show the sentence they have to agree to:

```rust
capabilities: vec![Capability::Network(vec![String::from("api.github.com")])],
```

Nothing is granted by asking. A request outside what was granted comes back refused, carrying
the sentence the dialog would have said — so a plugin can tell somebody what to allow rather
than that something went wrong.

## Publishing it

1. Tag a version. CI builds `plugin.wasm` and attaches it to the release.
2. Open a pull request against
   [the registry](https://github.com/theguriev/crook-plugins) adding one four-line
   `plugin.toml` that names your repository and the commit to build.
3. The registry builds the artifact itself, from that commit, reads it with the terminal's own
   reader, and publishes it in the list every Crook fetches. Nobody installs a binary a reviewer
   did not see built.

Anybody can also install it by hand, with no registry involved:

```sh
crook --install-plugin target/wasm32-unknown-unknown/release/plugin.wasm
```

## The vocabulary

[`crook_plugin_api`](https://docs.rs/crook_plugin_api) is the whole API, and its docs are the
reference. `0.8` is Cargo's way of writing "plugin ABI 8": the crate is versioned
`0.<abi>.<patch>`, a host loads exactly one ABI, and there is no wider range to depend on.

Four plugins to read when this one stops being enough:
[emoji](https://github.com/theguriev/crook-emoji) (a mark per tab, no permissions),
[worktree](https://github.com/theguriev/crook-worktree) (one capability),
[markdown](https://github.com/theguriev/crook-markdown) (a menu on a command, and the clipboard),
[pirate](https://github.com/theguriev/crook-pirate) (a network host, files, a panel and a clock).
