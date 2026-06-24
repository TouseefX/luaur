//! Faithful rustyline analog of Repl.cpp's `icGetCompletions`. Mirrors
//! `luau-repl-cli/src/functions/ic_get_completions.rs`, adapted to the cli-test
//! `get_completions` surface (which takes `&String` + a `&mut dyn FnMut`
//! callback).
//!
//! In C++ this bridged isocline's completion environment to the REPL's
//! `getCompletions`, calling `ic_add_completion_ex(cenv, completion, display, …)`
//! for each match. With rustyline the "completion environment" becomes a
//! `Vec<Pair>`: the `replacement` is the full completed word (what isocline used
//! as the inserted completion text) and the `display` is the matched key (the
//! human-readable alternative that isocline displayed when listing matches).

use alloc::string::String;
use alloc::vec::Vec;

use rustyline::completion::Pair;

use luaur_vm::records::lua_state::lua_State;

use crate::functions::get_completions::get_completions;

pub unsafe fn ic_get_completions(l: *mut lua_State, edit_buffer: &str) -> Vec<Pair> {
    let mut completions: Vec<Pair> = Vec::new();
    let buffer: String = edit_buffer.into();

    get_completions(l, &buffer, &mut |completion: &str, display: &str| {
        completions.push(Pair {
            display: display.into(),
            replacement: completion.into(),
        });
    });

    completions
}
