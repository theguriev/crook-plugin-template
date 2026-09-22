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
assets/icon.png                 its face, beside its name on the Plugins page and in the Store
assets/header.png               what it looks like, shown on a press
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

## Pictures

The icon and the preview travel *inside* `plugin.wasm`, as custom sections — two lines in
`lib.rs` put them there, and nothing beside the file has to be kept in step with it:

```rust
crook_plugin_api::icon!("../../../assets/icon.png");
crook_plugin_api::preview!(1, "../../../assets/header.png", "The chip in the header");
```

An icon is a square PNG, 32 to 256 pixels a side and at most 32 KiB; draw it at 128. A preview is
a PNG of at most 512 KiB with no side past 2048, captured at 2x — `crook-dev --snapshot` renders
at that scale, so a screenshot's logical size is its pixels halved — and its caption is one line.
Up to six, numbered. `crook-plugin-info plugin.wasm` prints what a module carries, and
`crook --dev-plugin plugin.wasm` draws it on the Plugins page with nothing installed.

## Asking for something

This asks for nothing, and a plugin that asks for nothing draws from the moment it is installed.
Everything else is a `Capability` in the manifest — one host, one path, one command, never a
category — and a person answers for it on the Plugins page before your plugin reaches any of it.
Add one and their card will show the sentence they have to agree to:

```rust
// in the `use` at the top of lib.rs, beside the shapes already there:
use crook_plugin_api::Capability;

// and in `manifest()`, instead of `Vec::new()`:
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

## Releasing

A release is a tag, and the tag is cut by a script:

```sh
./script/release 0.1.1 --push
```

It sets the version in `Cargo.toml`, writes the `## v0.1.1` section of `CHANGELOG.md`
from the commit titles since the previous tag with
[changelogen](https://github.com/unjs/changelogen), commits both as `chore(release):
v0.1.1`, tags it and pushes. `ci.yml` builds `plugin.wasm` from that tag and puts it on a
release page whose notes are that same section — written once, not once for the file and
again for the page. `--dry-run` prints the section and stops; `--push` is what starts the
build.

Which makes commit titles the release notes, so they are [Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/) — `feat(panel): …`, `fix: …`, the
types listed under `types` in `changelog.config.json`. A title in any other shape is not an
error to the generator, it is dropped without a word, so it is refused where it is still
easy to fix: `git config core.hooksPath script/hooks` installs the hook, and `commits.yml`
runs the same check on every pull request.

One line in `changelog.config.json` is this repository's name — change `repo` to yours when
you copy the template, or the compare links in your release notes will point back here.

## The vocabulary

[`crook_plugin_api`](https://docs.rs/crook_plugin_api) is the whole API, and its docs are the
reference. `0.8` is Cargo's way of writing "plugin ABI 8": the crate is versioned
`0.<abi>.<patch>`, a host loads exactly one ABI, and there is no wider range to depend on.

Four plugins to read when this one stops being enough:
[emoji](https://github.com/theguriev/crook-emoji) (a mark per tab, no permissions),
[worktree](https://github.com/theguriev/crook-worktree) (one capability),
[markdown](https://github.com/theguriev/crook-markdown) (a menu on a command, and the clipboard),
[pirate](https://github.com/theguriev/crook-pirate) (a network host, files, a panel and a clock).
