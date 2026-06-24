use crate::type_aliases::instruction_ir_translation::Instruction;

#[allow(non_snake_case)]
pub unsafe fn VM_PATCH_E(pc: *mut Instruction, slot: i32) {
    *pc = ((slot as u32) << 8) | (0x000000ffu32 & *pc);
}
