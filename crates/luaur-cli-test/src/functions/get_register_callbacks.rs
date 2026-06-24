use crate::type_aliases::register_callback::RegisterCallback;
use std::collections::HashSet;
use std::sync::OnceLock;

#[allow(non_snake_case)]
pub fn get_register_callbacks() -> &'static mut HashSet<RegisterCallback> {
    static mut CBS: OnceLock<HashSet<RegisterCallback>> = OnceLock::new();

    unsafe {
        // In C++, this is a local static initialized on first call.
        // In Rust, we use OnceLock to ensure it is initialized exactly once.
        // Since the C++ function returns a non-const reference and is used for registration,
        // we provide a static mut reference.
        if CBS.get().is_none() {
            let _ = CBS.set(HashSet::new());
        }
        // Safety: This follows the C++ pattern where the static is initialized once.
        // In a CLI test runner context, this is typically called during a single-threaded
        // initialization phase or via global constructors.
        CBS.get_mut().unwrap_unchecked()
    }
}
