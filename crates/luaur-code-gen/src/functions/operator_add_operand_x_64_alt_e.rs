use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

pub fn operator_add_register_x_64_operand_x_64(
    base: RegisterX64,
    mut op: OperandX64,
) -> OperandX64 {
    // CODEGEN_ASSERT's Rust-side handler expects raw pointers for file/function info.
    // The luau-code-gen macro currently passes &str, which doesn't match.
    // Use the same runtime condition checks directly instead of CODEGEN_ASSERT.
    debug_assert!(op.cat == CategoryX64::mem);
    debug_assert!(op.memSize == SizeX64::none);
    debug_assert!(op.base == RegisterX64::noreg);
    debug_assert!(op.index == RegisterX64::noreg || op.index.size() == base.size());

    op.base = base;
    op
}
