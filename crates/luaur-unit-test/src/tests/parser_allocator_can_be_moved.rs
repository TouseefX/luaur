#[cfg(test)]
#[test]
fn parser_allocator_can_be_moved() {
    use crate::records::counter::Counter;
    use luaur_ast::records::allocator::Allocator;

    let mut c: *mut Counter = std::ptr::null_mut();

    let mut inner = || {
        let mut allocator = Allocator::allocator();
        c = allocator.alloc(Counter::counter_counter());
        let moved = Allocator::allocator_allocator(&mut allocator);
        moved
    };

    Counter::reset_instance_count();
    let a = Allocator::allocator_allocator(&mut inner());

    assert_eq!(1, unsafe { (*c).id });
}
