use crate::enums::kind_a_64::KindA64;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::set::Set;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

impl IrRegAllocA64 {
    pub(crate) fn get_set(&mut self, kind: KindA64) -> &mut Set {
        match kind {
            KindA64::x | KindA64::w => &mut self.gpr,

            KindA64::s | KindA64::d | KindA64::q => &mut self.simd,

            _ => {
                debug_assert!(false, "Unexpected register kind");
                LUAU_UNREACHABLE!();
            }
        }
    }
}
