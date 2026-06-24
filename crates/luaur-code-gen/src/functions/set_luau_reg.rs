use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

use crate::enums::category_x_64::CategoryX64;
use crate::functions::luau_reg::luau_reg;

pub fn set_luau_reg(build: &mut AssemblyBuilderX64, tmp: RegisterX64, ri: i32, op: OperandX64) {
    debug_assert!(op.cat == CategoryX64::mem);

    build.vmovups(OperandX64::reg(tmp), op);
    build.vmovups(luau_reg(ri), OperandX64::reg(tmp));
}
