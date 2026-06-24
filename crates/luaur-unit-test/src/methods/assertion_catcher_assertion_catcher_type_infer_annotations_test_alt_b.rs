use crate::records::assertion_catcher::AssertionCatcher;
use luaur_common::functions::assert_handler::assert_handler;

impl Drop for AssertionCatcher {
    fn drop(&mut self) {
        *assert_handler() = self.oldhook;
    }
}
