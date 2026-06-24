use crate::records::repl_fixture::ReplFixture;
use crate::type_aliases::completion_set::CompletionSet;

use crate::functions::get_completions::get_completions;
use crate::records::completion::Completion;
use alloc::string::ToString;
use luaur_vm::functions::lua_gettop::lua_gettop;

impl ReplFixture {
    pub fn get_completion_set(&mut self, input_prefix: &str) -> CompletionSet {
        let mut result = CompletionSet::default();

        let top = unsafe { lua_gettop(self.l as *mut _) };

        let edit_buffer = input_prefix.to_string();
        let mut callback = |completion: &str, display: &str| {
            result.insert(Completion {
                completion: completion.to_string(),
                display: display.to_string(),
            });
        };

        get_completions(self.l as *mut _, &edit_buffer, &mut callback);

        debug_assert!(top == unsafe { lua_gettop(self.l as *mut _) });

        result
    }
}
