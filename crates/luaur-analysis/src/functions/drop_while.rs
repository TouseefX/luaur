pub fn drop_while<T, P>(vec: &mut alloc::vec::Vec<T>, pred: P)
where
    P: Fn(&T) -> bool,
{
    let mut it = 0;
    while it < vec.len() && pred(&vec[it]) {
        it += 1;
    }
    if it > 0 {
        vec.drain(0..it);
    }
}
