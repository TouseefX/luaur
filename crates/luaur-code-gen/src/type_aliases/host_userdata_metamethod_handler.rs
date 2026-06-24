use crate::enums::host_metamethod::HostMetamethod;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

#[allow(non_camel_case_types)]
pub type HostUserdataMetamethodHandler = Option<
    unsafe extern "C" fn(
        builder: *mut IrBuilder,
        lhs_ty: u8,
        rhs_ty: u8,
        result_reg: i32,
        lhs: IrOp,
        rhs: IrOp,
        method: HostMetamethod,
        pcpos: i32,
    ) -> bool,
>;
