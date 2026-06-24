use crate::records::native_stack_guard::NativeStackGuard;
use luaur_common::{FFlag, FInt};

impl NativeStackGuard {
    #[inline]
    pub fn is_ok(&self) -> bool {
        if !FFlag::LuauUseNativeStackGuard.get() || FInt::LuauStackGuardThreshold.get() <= 0 {
            return true;
        }

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let probe: u8 = 0;
            let sp = &probe as *const u8 as usize;
            let remaining = sp.wrapping_sub(self.low);

            return remaining > FInt::LuauStackGuardThreshold.get() as usize;
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        true
    }
}
