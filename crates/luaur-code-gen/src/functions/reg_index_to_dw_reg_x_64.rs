use crate::macros::dw_reg_x_64_rax::DW_REG_X64_RAX;
use crate::macros::dw_reg_x_64_rbp::DW_REG_X64_RBP;
use crate::macros::dw_reg_x_64_rbx::DW_REG_X64_RBX;
use crate::macros::dw_reg_x_64_rcx::DW_REG_X64_RCX;
use crate::macros::dw_reg_x_64_rdi::DW_REG_X64_RDI;
use crate::macros::dw_reg_x_64_rdx::DW_REG_X64_RDX;
use crate::macros::dw_reg_x_64_rsi::DW_REG_X64_RSI;
use crate::macros::dw_reg_x_64_rsp::DW_REG_X64_RSP;

pub fn reg_index_to_dw_reg_x_64(index: u8) -> i32 {
    match index {
        0 => DW_REG_X64_RAX,
        1 => DW_REG_X64_RCX,
        2 => DW_REG_X64_RDX,
        3 => DW_REG_X64_RBX,
        4 => DW_REG_X64_RSP,
        5 => DW_REG_X64_RBP,
        6 => DW_REG_X64_RSI,
        7 => DW_REG_X64_RDI,
        8..=15 => index as i32,
        _ => panic!("invalid x64 register index {index}"),
    }
}
