use crate::enums::ir_const_kind::IrConstKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::IrConst;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_tag(&mut self, value: u8) -> IrOp {
        let constant: IrConst = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut constant = constant;
        constant.kind = IrConstKind::Tag;
        constant.value.value_tag = value;
        self.const_any(constant, value as u64)
    }
}
