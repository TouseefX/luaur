use crate::type_aliases::register_callback::RegisterCallback;
use std::collections::HashSet;
use std::sync::OnceLock;

pub fn get_register_callbacks() -> &'static mut HashSet<RegisterCallback> {
    static mut INSTANCE: Option<HashSet<RegisterCallback>> = None;

    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(HashSet::new());
        }
        INSTANCE.as_mut().unwrap()
    }
}
