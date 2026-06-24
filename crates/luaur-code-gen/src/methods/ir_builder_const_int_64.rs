use crate::enums::ir_const_kind::IrConstKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::{IrConst, IrConstValue};
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_int_64(&mut self, value: i64) -> IrOp {
        let mut constant = IrConst {
            kind: IrConstKind::Int64,
            value: IrConstValue { value_int64: value },
        };
        self.const_any(constant, value as u64)
    }
}
