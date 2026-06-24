#[macro_export]
#[allow(non_snake_case)]
macro_rules! TEST_REGISTER {
    ($der:ident, $reg:ident, $run:ident) => {
        #[allow(non_snake_case)]
        fn $run() {
            let mut fix = $der::default();
            fix.test();
        }

        #[allow(non_snake_case)]
        fn $reg() {
            static mut REGISTERED: bool = false;
            unsafe {
                if !REGISTERED {
                    $crate::addTestCallback($run);
                    REGISTERED = true;
                }
            }
        }
    };
}

pub use TEST_REGISTER;
