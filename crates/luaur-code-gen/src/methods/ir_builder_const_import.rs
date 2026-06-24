use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::IrConst;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_import(&mut self, value: u32) -> IrOp {
        let constant = IrConst {
            kind: crate::enums::ir_const_kind::IrConstKind::Import,
            value: crate::records::ir_const::IrConstValue { value_uint: value },
        };

        self.const_any(constant, value as u64)
    }
}
