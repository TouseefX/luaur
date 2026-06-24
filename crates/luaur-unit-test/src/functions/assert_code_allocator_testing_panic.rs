pub fn assert_code_allocator_testing_panic(payload: Box<dyn core::any::Any + Send>) {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        assert_eq!(*message, "testing");
    } else if let Some(message) = payload.downcast_ref::<String>() {
        assert_eq!(message, "testing");
    } else {
        panic!("unexpected panic payload");
    }
}
