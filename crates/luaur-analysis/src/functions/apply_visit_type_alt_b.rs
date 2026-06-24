pub fn apply<A, B, C, F, R>(tid: A, t: &B, _unused: &mut C, f: &mut F) -> R
where
    F: FnMut(A, &B) -> R,
{
    f(tid, t)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use apply as apply_mut;
