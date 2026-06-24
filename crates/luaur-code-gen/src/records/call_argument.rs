use crate::enums::size_x_64::SizeX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CallArgument {
    pub target_size: SizeX64,
    pub source: OperandX64,
    pub source_op: IrOp,
    pub target: OperandX64,
    pub candidate: bool,
}

impl Default for CallArgument {
    fn default() -> Self {
        Self {
            target_size: SizeX64::none,
            source: OperandX64 {
                cat: crate::enums::category_x_64::CategoryX64::reg,
                index: crate::records::register_x_64::RegisterX64::noreg,
                base: crate::records::register_x_64::RegisterX64::noreg,
                memSize: SizeX64::none,
                scale: 1,
                imm: 0,
            },
            source_op: IrOp::default(),
            target: OperandX64 {
                cat: crate::enums::category_x_64::CategoryX64::reg,
                index: crate::records::register_x_64::RegisterX64::noreg,
                base: crate::records::register_x_64::RegisterX64::noreg,
                memSize: SizeX64::none,
                scale: 1,
                imm: 0,
            },
            candidate: true,
        }
    }
}
