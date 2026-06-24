use crate::records::scoped_exit::ScopedExit;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ScopedExit {
    pub fn scoped_exit_function_void(f: alloc::boxed::Box<dyn FnOnce()>) -> Self {
        let func = Some(f);
        LUAU_ASSERT!(func.is_some());
        Self { func }
    }
}
