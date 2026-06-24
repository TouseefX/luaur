use crate::records::test::Test;

impl Test {
    pub fn new() -> Self {
        crate::records::test::TEST_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        Self { x: 0, y: 0.0 }
    }
}
