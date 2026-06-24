use alloc::vec::Vec;
use luaur_common::enums::luau_proto_flag::LuauProtoFlag;
use luaur_vm::functions::lua_a_toobject::luaA_toobject;
use luaur_vm::functions::lua_is_lfunction::lua_is_lfunction;
use luaur_vm::macros::clvalue::clvalue;
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::lua_state::lua_State;
use luaur_vm::type_aliases::t_value::TValue;

use crate::enums::code_gen_flags::CodeGenFlags;
use crate::functions::gather_functions::gather_functions;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::function_bytecode_summary::FunctionBytecodeSummary;

pub fn summarize_bytecode(
    L: *mut lua_State,
    idx: i32,
    nesting_limit: u32,
) -> Vec<FunctionBytecodeSummary> {
    unsafe {
        CODEGEN_ASSERT!(lua_is_lfunction(L, idx) != 0);
        let func: *const TValue = luaA_toobject(L, idx);

        let cl = clvalue!(func);
        let root: *mut Proto = (*(*cl).inner.l).p;

        let mut protos: Vec<*mut Proto> = Vec::new();
        gather_functions(
            &mut protos,
            root,
            CodeGenFlags::CodeGen_ColdFunctions as u32,
            ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
        );

        let mut summaries: Vec<FunctionBytecodeSummary> = Vec::with_capacity(protos.len());

        for proto in protos {
            if !proto.is_null() {
                summaries.push(FunctionBytecodeSummary::from_proto(proto, nesting_limit));
            }
        }

        summaries
    }
}
