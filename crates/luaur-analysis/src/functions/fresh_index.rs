use core::sync::atomic::{AtomicI32, Ordering};

static NEXT_INDEX: AtomicI32 = AtomicI32::new(0);

#[allow(non_snake_case)]
pub fn fresh_index() -> i32 {
    NEXT_INDEX.fetch_add(1, Ordering::Relaxed) + 1
}
