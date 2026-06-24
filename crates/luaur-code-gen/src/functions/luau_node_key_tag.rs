use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

// kOffsetOfTKeyTagNext = 12 (from EmitCommon.h)
// offsetof(LuaNode, key) = 16 (LuaNode is 32 bytes, first 16 bytes are val, then key)
// So the offset is 16 + 12 = 28
// We use dword[base + disp] form
pub const fn luau_node_key_tag(node: RegisterX64) -> OperandX64 {
    OperandX64::mem(SizeX64::dword, RegisterX64::noreg, 0, node, 28)
}
