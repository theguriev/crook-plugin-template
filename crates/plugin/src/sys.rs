//! The doors out of the sandbox, and the stubs that stand in for them.
//!
//! A plugin has no filesystem, no network, no clock and no thread. What it has
//! is the handful of functions the host imports into it, and the two this one
//! uses are here.
//!
//! They are wrapped rather than called from the logic for one reason:
//! everything else in this crate then builds and runs on an ordinary machine,
//! so what the plugin *draws* is decided by something `cargo test` can run
//! with no terminal to install it into.

/// How loud a line to the host's log is.
///
/// All three are here even though this plugin only says `Info`, because the
/// first thing anybody writes after "hello" is the line that says what went
/// wrong.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "the template says Info; a real plugin says the others"
)]
pub enum Level {
    /// Something failed.
    Error = 1,
    /// Something is not right but nothing failed.
    Warn = 2,
    /// Worth knowing.
    Info = 3,
}

#[cfg(target_arch = "wasm32")]
mod imports {
    #[link(wasm_import_module = "crook")]
    unsafe extern "C" {
        pub fn contribute(
            slot: *const u8,
            slot_len: usize,
            entry: *const u8,
            entry_len: usize,
            order: i32,
        );
        pub fn log(level: i32, text: *const u8, len: usize);
    }
}

/// Puts something in a slot the host declares.
///
/// `order` is where this goes among everything else in that slot, and lower is
/// earlier. A slot that holds one thing is decided by the order alone, so a
/// plugin that means to replace what is there asks for a smaller number.
pub fn contribute(slot: &str, entry: &str, order: i32) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        imports::contribute(
            slot.as_ptr(),
            slot.len(),
            entry.as_ptr(),
            entry.len(),
            order,
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (slot, entry, order);
    }
}

/// Says something in the host's log, which is how a plugin is heard by whoever
/// is running it when it has nothing to draw.
pub fn log(level: Level, text: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        imports::log(level as i32, text.as_ptr(), text.len());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (level, text);
    }
}
