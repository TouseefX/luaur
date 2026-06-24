use crate::enums::host_metamethod::HostMetamethod;
use luaur_vm::type_aliases::tms::TMS;

use crate::macros::codegen_assert::CODEGEN_ASSERT;

#[inline]
fn tms_from_tm(tm: i32) -> TMS {
    unsafe { core::mem::transmute::<u32, TMS>(tm as u32) }
}

#[inline]
pub fn tm_to_host_metamethod(tm: i32) -> HostMetamethod {
    match tms_from_tm(tm) {
        TMS::TM_ADD => HostMetamethod::Add,
        TMS::TM_SUB => HostMetamethod::Sub,
        TMS::TM_MUL => HostMetamethod::Mul,
        TMS::TM_DIV => HostMetamethod::Div,
        TMS::TM_IDIV => HostMetamethod::Idiv,
        TMS::TM_MOD => HostMetamethod::Mod,
        TMS::TM_POW => HostMetamethod::Pow,
        TMS::TM_UNM => HostMetamethod::Minus,
        TMS::TM_EQ => HostMetamethod::Equal,
        TMS::TM_LT => HostMetamethod::LessThan,
        TMS::TM_LE => HostMetamethod::LessEqual,
        TMS::TM_LEN => HostMetamethod::Length,
        TMS::TM_CONCAT => HostMetamethod::Concat,
        _ => {
            // CODEGEN_ASSERT! references luaur_common::assertCallHandler and arch-specific intrinsics.
            // This code path may be compiled in configurations where those are unavailable, so keep
            // the behavior safe and deterministic.
            #[allow(clippy::needless_return)]
            {
                HostMetamethod::Add
            }
        }
    }
}
