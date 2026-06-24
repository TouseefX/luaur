use crate::type_aliases::register_callback::RegisterCallback;
use alloc::collections::BTreeSet;

pub(crate) fn get_register_callbacks() -> &'static mut BTreeSet<RegisterCallback> {
    static mut CBS: *mut BTreeSet<RegisterCallback> = core::ptr::null_mut();

    unsafe {
        if CBS.is_null() {
            let cbs = Box::new(BTreeSet::<RegisterCallback>::new());
            CBS = Box::into_raw(cbs);
        }
        &mut *CBS
    }
}
