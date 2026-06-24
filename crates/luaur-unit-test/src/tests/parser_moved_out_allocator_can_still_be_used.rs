#[cfg(test)]
#[test]
fn parser_moved_out_allocator_can_still_be_used() {
    use luaur_ast::records::allocator::Allocator;

    let mut outer = Allocator::allocator();
    let mut inner = Allocator::allocator_allocator(&mut outer);

    // NOLINTNEXTLINE(bugprone-use-after-move) -- verifying moved-from state
    let i = outer.alloc::<i32>(55);
    assert!(!i.is_null());
    unsafe {
        assert_eq!(*i, 55);
    }
}
