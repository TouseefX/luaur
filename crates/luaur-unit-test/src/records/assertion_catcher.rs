use core::sync::atomic::AtomicI32;
use luaur_common::type_aliases::assert_handler::AssertHandler;

pub(crate) static ASSERTION_CATCHER_TRIPPED: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone)]
pub struct AssertionCatcher {
    pub(crate) oldhook: AssertHandler,
}
