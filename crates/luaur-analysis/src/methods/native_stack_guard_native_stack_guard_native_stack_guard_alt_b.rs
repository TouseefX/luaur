use crate::records::native_stack_guard::NativeStackGuard;
use luaur_common::FFlag;

impl NativeStackGuard {
    pub fn native_stack_guard(&mut self) {
        self.high = 0;
        self.low = 0;

        if !FFlag::LuauUseNativeStackGuard.get() {
            return;
        }

        #[cfg(target_os = "windows")]
        {
            #[allow(non_snake_case)]
            extern "system" {
                fn GetCurrentThreadStackLimits(low_limit: *mut usize, high_limit: *mut usize);
            }

            unsafe {
                GetCurrentThreadStackLimits(&mut self.low, &mut self.high);
            }
        }

        #[cfg(target_os = "macos")]
        {
            extern "C" {
                fn pthread_self() -> *mut core::ffi::c_void;
                fn pthread_get_stackaddr_np(
                    thread: *mut core::ffi::c_void,
                ) -> *mut core::ffi::c_void;
                fn pthread_get_stacksize_np(thread: *mut core::ffi::c_void) -> usize;
            }

            unsafe {
                let thread = pthread_self();
                let addr = pthread_get_stackaddr_np(thread) as usize;
                let size = pthread_get_stacksize_np(thread);
                self.low = addr.saturating_sub(size);
                self.high = addr;
            }
        }
    }
}
