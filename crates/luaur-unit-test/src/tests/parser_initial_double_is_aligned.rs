#[cfg(test)]
#[test]
fn parser_initial_double_is_aligned() {
    use luaur_ast::records::allocator::Allocator;

    let mut alloc = Allocator::allocator();
    let one = alloc.alloc::<f64>(0.0);
    let addr = one as usize;
    let align_mask = std::mem::align_of::<f64>() - 1;
    assert_eq!(addr & align_mask, 0);
}
