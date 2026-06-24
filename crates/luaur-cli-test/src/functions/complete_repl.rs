//! Faithful rustyline analog of Repl.cpp's `completeRepl`. Mirrors
//! `luau-repl-cli/src/functions/complete_repl.rs`, adapted to the cli-test
//! module layout (`lua_State` under `records::lua_state`; the local
//! `is_method_or_function_char` takes a `c_long` length).
//!
//! In C++ this was:
//!     ic_complete_word(cenv, editBuffer, icGetCompletions, isMethodOrFunctionChar);
//! `ic_complete_word` walks backwards from the cursor over characters for which
//! the `is_word_char` predicate (`isMethodOrFunctionChar`) returns true to find
//! the start of the word being completed, hands that word to the completer
//! (`icGetCompletions`), and reports completions relative to that word start.
//!
//! rustyline's `Completer::complete(line, pos, ctx)` gives us the full line and
//! cursor position and expects `(start, candidates)`, so we reproduce isocline's
//! word-boundary scan here and return the matches gathered by `ic_get_completions`.

use alloc::vec::Vec;
use core::ffi::{c_char, c_long};

use rustyline::completion::Pair;

use luaur_vm::records::lua_state::lua_State;

use crate::functions::ic_get_completions::ic_get_completions;
use crate::functions::is_method_or_function_char::is_method_or_function_char;

pub unsafe fn complete_repl(l: *mut lua_State, line: &str, pos: usize) -> (usize, Vec<Pair>) {
    let bytes = line.as_bytes();

    // Walk backwards from the cursor over method/function characters
    // (alphanumeric, '.', ':', '_') to locate the start of the word, exactly
    // as isocline's word-boundary detection (`isMethodOrFunctionChar`) does.
    let mut start = pos;
    while start > 0 {
        let c = bytes[start - 1] as c_char;
        if is_method_or_function_char(&c as *const c_char, 1 as c_long) {
            start -= 1;
        } else {
            break;
        }
    }

    let edit_buffer = &line[start..pos];
    let completions = ic_get_completions(l, edit_buffer);

    (start, completions)
}
