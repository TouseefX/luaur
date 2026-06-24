use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

#[inline]
pub fn luau_node_key_value(node: RegisterX64) -> OperandX64 {
    // offsetof(LuaNode, key) = 16 (LuaNode is 32 bytes, first 16 bytes are base, then key)
    // offsetof(TKey, value) = 0 (TKey starts with Value value)
    // So the offset is 16 + 0 = 16
    // We use qword[base + disp] form
    OperandX64::mem(SizeX64::qword, RegisterX64::noreg, 0, node, 16)
}
