pub fn countlz(n: u32) -> i32 {
    n.leading_zeros() as i32
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use countlz as countlz_u32;
