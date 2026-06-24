#[allow(non_snake_case)]
pub fn apply<F, A, B, C, R>(tid: A, t: &B, c: &mut C, mut f: F) -> R
where
    F: FnMut(A, &B, &mut C) -> R,
{
    f(tid, t, c)
}
