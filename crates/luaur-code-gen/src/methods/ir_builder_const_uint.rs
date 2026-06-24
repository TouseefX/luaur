use crate::enums::ir_const_kind::IrConstKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::IrConst;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_uint(&mut self, value: u32) -> IrOp {
        let mut constant: IrConst = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        constant.kind = IrConstKind::Uint;
        constant.value.value_uint = value;
        self.const_any(constant, value as u64)
    }
}
