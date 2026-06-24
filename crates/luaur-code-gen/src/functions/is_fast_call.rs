use luaur_common::enums::luau_opcode::LuauOpcode;

pub fn is_fast_call(op: LuauOpcode) -> bool {
    match op {
        LuauOpcode::LOP_FASTCALL
        | LuauOpcode::LOP_FASTCALL1
        | LuauOpcode::LOP_FASTCALL2
        | LuauOpcode::LOP_FASTCALL2K
        | LuauOpcode::LOP_FASTCALL3 => true,
        _ => false,
    }
}
