use crate::enums::ir_const_kind::IrConstKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::IrConst;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_double(&mut self, value: f64) -> IrOp {
        let constant = IrConst {
            kind: IrConstKind::Double,
            value: unsafe {
                core::mem::transmute::<u64, crate::records::ir_const::IrConstValue>(value.to_bits())
            },
        };

        self.const_any(constant, value.to_bits())
    }
}
