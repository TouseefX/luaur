use crate::records::native_stack_guard::NativeStackGuard;
use luaur_common::{FFlag, FInt};

impl NativeStackGuard {
    pub fn is_ok_3(&self) -> bool {
        if !FFlag::LuauUseNativeStackGuard.get() || FInt::LuauStackGuardThreshold.get() <= 0 {
            return true;
        }

        // C++: const uintptr_t sp = uintptr_t(__builtin_frame_address(0));
        // __builtin_frame_address(0) yields an address within the current
        // stack frame; the portable equivalent is the address of a stack local.
        let probe: u8 = 0;
        let sp = &probe as *const u8 as usize;

        let remaining = sp.wrapping_sub(self.low);

        remaining > FInt::LuauStackGuardThreshold.get() as usize
    }
}
