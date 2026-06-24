use crate::enums::abix_64::ABIX64;
use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

// Bytes for register 'home' locations that can be used by callees under Windows ABI.
const K_STACK_REG_HOME_STORAGE: i32 = 4 * 8;

pub(crate) const fn xmm(i: u8) -> RegisterX64 {
    RegisterX64 {
        bits: (i << RegisterX64::INDEX_SHIFT) | SizeX64::xmmword as u8,
    }
}

// static const std::array<OperandX64, 6> kWindowsGprOrder
pub(crate) fn windows_gpr_order() -> [OperandX64; 6] {
    [
        OperandX64::reg(RegisterX64::rcx),
        OperandX64::reg(RegisterX64::rdx),
        OperandX64::reg(RegisterX64::r8),
        OperandX64::reg(RegisterX64::r9),
        OperandX64::mem(
            SizeX64::none,
            RegisterX64::noreg,
            1,
            RegisterX64::rsp,
            K_STACK_REG_HOME_STORAGE,
        ),
        OperandX64::mem(
            SizeX64::none,
            RegisterX64::noreg,
            1,
            RegisterX64::rsp,
            K_STACK_REG_HOME_STORAGE + 8,
        ),
    ]
}

// static const std::array<OperandX64, 6> kSystemvGprOrder
pub(crate) fn systemv_gpr_order() -> [OperandX64; 6] {
    [
        OperandX64::reg(RegisterX64::rdi),
        OperandX64::reg(RegisterX64::rsi),
        OperandX64::reg(RegisterX64::rdx),
        OperandX64::reg(RegisterX64::rcx),
        OperandX64::reg(RegisterX64::r8),
        OperandX64::reg(RegisterX64::r9),
    ]
}

// static const std::array<OperandX64, 4> kXmmOrder
pub(crate) fn xmm_order() -> [OperandX64; 4] {
    [
        OperandX64::reg(xmm(0)),
        OperandX64::reg(xmm(1)),
        OperandX64::reg(xmm(2)),
        OperandX64::reg(xmm(3)),
    ]
}

impl IrCallWrapperX64 {
    pub fn get_next_argument_target(&self, size: SizeX64) -> OperandX64 {
        if size == SizeX64::xmmword {
            CODEGEN_ASSERT!((self.xmm_pos as usize) < xmm_order().len());
            return xmm_order()[self.xmm_pos as usize];
        }

        let gpr_order = if unsafe { (*self.build).abi } == ABIX64::Windows {
            windows_gpr_order()
        } else {
            systemv_gpr_order()
        };

        CODEGEN_ASSERT!((self.gpr_pos as usize) < gpr_order.len());
        let mut target = gpr_order[self.gpr_pos as usize];

        // Keep requested argument size
        if target.cat == CategoryX64::reg {
            target.base = RegisterX64 {
                bits: (target.base.bits & RegisterX64::INDEX_MASK) | size as u8,
            };
        } else if target.cat == CategoryX64::mem {
            target.memSize = size;
        }

        target
    }
}
