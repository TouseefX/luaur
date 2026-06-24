#[macro_export]
#[allow(non_snake_case)]
macro_rules! VM_INTERRUPT {
    ($L:expr) => {
        unsafe {
            let l_state = $L;
            let interrupt_fn = (*(*l_state).global).cb.interrupt;
            if luaur_common::LUAU_UNLIKELY!(!interrupt_fn.is_null()) {
                interrupt_fn(l_state, 0);
            }
        }
    };
}

pub use VM_INTERRUPT;
