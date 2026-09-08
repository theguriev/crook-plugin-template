//! What this plugin promises, without a terminal to install it into.
//!
//! Everything that decides what a person sees is a plain function over plain
//! values — which is the reason `sys.rs` exists — so the whole of it is
//! testable by `cargo test` on an ordinary machine.

use super::*;

#[test]
fn it_says_what_it_is_before_any_of_it_runs() {
    let manifest = manifest();

    assert_eq!(manifest.abi, ABI_VERSION);
    assert!(
        manifest.capabilities.is_empty(),
        "this one asks for nothing, and a plugin that asks for nothing draws at once"
    );
    assert_eq!(manifest.version, env!("CARGO_PKG_VERSION"));
}

#[test]
fn it_draws_a_word_in_the_slot_it_asked_for() {
    let drawn = tree(Some(Render {
        slot: String::from(SLOT),
        entry: String::from(ENTRY),
        subject: None,
    }));

    let Node::Row(row) = drawn else {
        panic!("a row is what this draws");
    };
    assert!(matches!(row.first(), Some(Node::Text { text, .. }) if text == "hello"));
}

#[test]
fn anything_else_draws_nothing_rather_than_a_guess() {
    // A slot this plugin did not contribute to, and a request that did not
    // decode. Drawing nothing is not a hole: the host puts back whatever it
    // would have drawn without a plugin there.
    let elsewhere = tree(Some(Render {
        slot: String::from("tab.row.mark"),
        entry: String::from(ENTRY),
        subject: None,
    }));

    assert_eq!(elsewhere, Node::Empty);
    assert_eq!(tree(None), Node::Empty);
}
