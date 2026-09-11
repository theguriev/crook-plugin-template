//! A Crook plugin that says hello in the header.
//!
//! Everything a plugin has to have and nothing it does not: the five exports
//! the host calls, a manifest that asks for no permission at all, and one
//! [`Node`] describing what to draw. Change the id, change the tree, and it is
//! yours.
//!
//! # The shape of it
//!
//! * [`crook_abi_version`] — which vocabulary this speaks. Checked first, and
//!   a mismatch is a refusal by number rather than a plugin that decodes a
//!   shape meaning something else now.
//! * [`crook_manifest`] — what this is and what it wants to be allowed to do.
//!   Read *before* any of it runs, which is what lets somebody see what it
//!   asks for and refuse it without running a line.
//! * [`crook_alloc`] — somewhere for the host to put bytes it is handing over.
//! * [`crook_build`] — where this puts itself. Called once when it loads.
//! * [`crook_render`] — what to draw, called when the host is drawing that
//!   slot.
//!
//! # It describes, it does not paint
//!
//! There is no colour and no pixel here. A [`Node`] says *what a thing is* —
//! a label, a badge, a row — and the host draws it in whatever theme is in
//! force, at whatever size that place calls for. That is what makes a plugin
//! written today come out right in a theme written years from now.
//!
//! # Asking for more
//!
//! This asks for nothing, and a plugin that asks for nothing draws from the
//! moment it is installed. Anything else — a file, a host, the clipboard — is
//! a [`Capability`] in the manifest, granted by a person on the Plugins page,
//! and refused until they do. Add one there and the card will show the
//! sentence they have to agree to.

use crook_plugin_api::{ABI_VERSION, Manifest, Node, Render, Size, Tone, from_bytes, to_bytes};

mod sys;

// The plugin's face, and what it looks like, for the Plugins page and the
// Store. Inside the module rather than beside it, for the reason a plugin is
// one file: what says what the plugin is travels with it. Custom sections,
// not data — they cost no memory and no fuel. The icon is a 128 px square;
// a preview is a screenshot captured at 2x, with a caption of one line.
crook_plugin_api::icon!("../../../assets/icon.png");
crook_plugin_api::preview!(1, "../../../assets/header.png", "The chip in the header");

/// Where this draws: the right-hand end of the header.
///
/// One of the slots the host declares. `tab.row.mark` is a mark on every tab,
/// `pane.chips` is the row under what you are typing, `block.menu` is the menu
/// on a finished command — a plugin puts itself in the place it belongs.
const SLOT: &str = "header.right";

/// What this contribution is called, among this plugin's own.
const ENTRY: &str = "hello";

/// Where it goes among everything else in that slot. Lower is earlier.
const ORDER: i32 = 0;

/// The bytes handed back to the host, kept alive until the next call.
///
/// A guest returns a pointer and a length, so what it points at has to outlive
/// the return. One buffer, replaced each time: the host copies before it asks
/// again.
static mut ANSWER: Vec<u8> = Vec::new();

/// Hands `bytes` to the host as a packed pointer and length.
fn hand_back(bytes: Vec<u8>) -> i64 {
    // SAFETY: a plugin is called on one thread, and never re-entered — the
    // host has the answer copied out before it calls in again.
    unsafe {
        let answer = &raw mut ANSWER;
        (*answer) = bytes;
        (((*answer).as_ptr() as u64) << 32 | (*answer).len() as u64) as i64
    }
}

/// Which version of the vocabulary this was built against.
#[unsafe(no_mangle)]
pub extern "C" fn crook_abi_version() -> i32 {
    ABI_VERSION as i32
}

/// Somewhere for the host to put the bytes it is handing over.
#[unsafe(no_mangle)]
pub extern "C" fn crook_alloc(length: i32) -> i32 {
    let Ok(layout) = std::alloc::Layout::from_size_align(length.max(1) as usize, 1) else {
        return 0;
    };
    // SAFETY: a non-zero size, and a layout built for it.
    unsafe { std::alloc::alloc(layout) as i32 }
}

/// Copies out what the host wrote, and gives the memory back.
///
/// SAFETY: `pointer` and `length` must be what a previous [`crook_alloc`]
/// answered and what the host wrote into.
unsafe fn take(pointer: i32, length: i32) -> Vec<u8> {
    if pointer <= 0 || length < 0 {
        return Vec::new();
    }
    // SAFETY: the host wrote `length` bytes at `pointer` before calling in.
    let bytes =
        unsafe { std::slice::from_raw_parts(pointer as *const u8, length as usize) }.to_vec();
    // SAFETY: the layout `crook_alloc` used.
    unsafe {
        std::alloc::dealloc(
            pointer as *mut u8,
            std::alloc::Layout::from_size_align_unchecked(length.max(1) as usize, 1),
        );
    }
    bytes
}

/// What this plugin is, and what it asks to be allowed to do.
#[unsafe(no_mangle)]
pub extern "C" fn crook_manifest() -> i64 {
    hand_back(to_bytes(&manifest()).unwrap_or_default())
}

/// The manifest as a value, so a test can read it.
pub fn manifest() -> Manifest {
    Manifest {
        abi: ABI_VERSION,
        // `owner/name`: the owner is your GitHub account, and the pair is what
        // a grant, a keybinding and a registry entry all name. Change it
        // before you publish — two plugins cannot share one.
        id: String::from("you/hello"),
        name: String::from("Hello"),
        description: String::from("A word in the header, and the shortest plugin there is."),
        version: String::from(env!("CARGO_PKG_VERSION")),
        // Nothing. Add a `Capability` here and a person answers for it on the
        // Plugins page before this reaches any of it.
        capabilities: Vec::new(),
    }
}

/// Where this puts itself, once, when it loads.
#[unsafe(no_mangle)]
pub extern "C" fn crook_build() -> i32 {
    sys::contribute(SLOT, ENTRY, ORDER);
    sys::log(sys::Level::Info, "hello from a plugin");
    0
}

/// What to draw.
#[unsafe(no_mangle)]
pub extern "C" fn crook_render(pointer: i32, length: i32) -> i64 {
    // SAFETY: the host allocated and wrote this before calling in.
    let bytes = unsafe { take(pointer, length) };
    hand_back(to_bytes(&tree(from_bytes::<Render>(&bytes).ok())).unwrap_or_default())
}

/// What one render comes to.
///
/// A function over the request rather than a body inside the export, so that
/// "what does it draw when the host asks about something else" is a question a
/// test can ask. Anything unexpected draws **nothing** rather than a guess:
/// the host then draws whatever it would have drawn without a plugin.
pub fn tree(render: Option<Render>) -> Node {
    let Some(render) = render else {
        return Node::Empty;
    };
    if render.slot != SLOT {
        return Node::Empty;
    }

    Node::Row(vec![
        Node::Text {
            text: String::from("hello"),
            size: Size::Small,
            tone: Tone::Muted,
        },
        Node::Badge {
            text: String::from("plugin"),
            tone: Tone::Accent,
        },
    ])
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
