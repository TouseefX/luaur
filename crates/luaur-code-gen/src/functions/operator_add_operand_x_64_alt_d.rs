use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

pub fn operator_add_operand_x_64_register_x_64(op: OperandX64, base: RegisterX64) -> OperandX64 {
    debug_assert!(op.cat == CategoryX64::mem);
    debug_assert!(op.memSize == SizeX64::none);
    debug_assert!(op.base == RegisterX64::noreg);
    debug_assert!(op.index == RegisterX64::noreg || op.index.size() == base.size());

    let mut op = op;
    op.base = base;
    op
}
