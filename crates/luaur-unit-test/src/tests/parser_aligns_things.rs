#[cfg(test)]
#[test]
fn parser_aligns_things() {
    use luaur_ast::records::allocator::Allocator;

    let mut alloc = Allocator::allocator();
    let _one = alloc.alloc(0u8);
    let two = alloc.alloc(0.0_f64);
    let align_mask = core::mem::align_of::<f64>() - 1;
    let two_addr = two as usize;
    assert_eq!(0, two_addr & align_mask);
}
