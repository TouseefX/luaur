use core::ffi::{c_char, c_int, c_void};

use crate::functions::compare_member_name::compare_member_name;
use crate::functions::luau_library_constant_lookup::luau_library_constant_lookup;
use crate::functions::luau_library_type_lookup::luau_library_type_lookup;
use crate::functions::userdata_access::userdata_access;
use crate::functions::userdata_access_bytecode_type::userdata_access_bytecode_type;
use crate::functions::userdata_metamethod::userdata_metamethod;
use crate::functions::userdata_metamethod_bytecode_type::userdata_metamethod_bytecode_type;
use crate::functions::userdata_namecall::userdata_namecall;
use crate::functions::userdata_namecall_bytecode_type::userdata_namecall_bytecode_type;
use crate::functions::vector_access::vector_access;
use crate::functions::vector_access_bytecode_type::vector_access_bytecode_type;
use crate::functions::vector_namecall::vector_namecall;
use crate::functions::vector_namecall_bytecode_type::vector_namecall_bytecode_type;
use crate::records::lowering_fixture::LoweringFixture;
use luaur_code_gen::enums::host_metamethod::HostMetamethod;
use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use luaur_code_gen::functions::set_userdata_remapper::set_userdata_remapper;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;
use luaur_compiler::type_aliases::compile_constant::CompileConstant;
use luaur_compiler::type_aliases::lua_compile_constant::lua_CompileConstant;
use luaur_vm::records::lua_state::lua_State;

pub(crate) unsafe extern "C" fn luau_library_type_lookup_callback(
    library: *const c_char,
    member: *const c_char,
) -> c_int {
    luau_library_type_lookup(library, member)
}

pub(crate) unsafe extern "C" fn luau_library_constant_lookup_callback(
    library: *const c_char,
    member: *const c_char,
    constant: *mut CompileConstant,
) {
    luau_library_constant_lookup(library, member, constant);
}

pub(crate) unsafe extern "C" fn luau_library_constant_lookup_c_callback(
    library: *const c_char,
    member: *const c_char,
    constant: *mut lua_CompileConstant,
) {
    luau_library_constant_lookup(library, member, constant as *mut CompileConstant);
}

pub(crate) unsafe extern "C" fn vector_access_bytecode_type_callback(
    member: *const c_char,
    member_length: usize,
) -> u8 {
    vector_access_bytecode_type(member, member_length)
}

pub(crate) unsafe extern "C" fn vector_namecall_bytecode_type_callback(
    member: *const c_char,
    member_length: usize,
) -> u8 {
    vector_namecall_bytecode_type(member, member_length)
}

pub(crate) unsafe extern "C" fn vector_access_callback(
    builder: *mut IrBuilder,
    member: *const c_char,
    member_length: usize,
    result_reg: i32,
    source_reg: i32,
    pcpos: i32,
) -> bool {
    vector_access(
        &mut *builder,
        member,
        member_length,
        result_reg,
        source_reg,
        pcpos,
    )
}

pub(crate) unsafe extern "C" fn vector_namecall_callback(
    builder: *mut IrBuilder,
    member: *const c_char,
    member_length: usize,
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
) -> bool {
    vector_namecall(
        &mut *builder,
        member,
        member_length,
        arg_res_reg,
        source_reg,
        params,
        results,
        pcpos,
    )
}

pub(crate) unsafe extern "C" fn userdata_access_bytecode_type_callback(
    r#type: u8,
    member: *const c_char,
    member_length: usize,
) -> u8 {
    userdata_access_bytecode_type(r#type, member, member_length)
}

pub(crate) unsafe extern "C" fn userdata_metamethod_bytecode_type_callback(
    lhs_ty: u8,
    rhs_ty: u8,
    method: HostMetamethod,
) -> u8 {
    userdata_metamethod_bytecode_type(lhs_ty, rhs_ty, method)
}

pub(crate) unsafe extern "C" fn userdata_namecall_bytecode_type_callback(
    r#type: u8,
    member: *const c_char,
    member_length: usize,
) -> u8 {
    userdata_namecall_bytecode_type(r#type, member, member_length)
}

pub(crate) unsafe extern "C" fn userdata_access_callback(
    builder: *mut IrBuilder,
    r#type: u8,
    member: *const c_char,
    member_length: usize,
    result_reg: i32,
    source_reg: i32,
    pcpos: i32,
) -> bool {
    userdata_access(
        &mut *builder,
        r#type,
        member,
        member_length,
        result_reg,
        source_reg,
        pcpos,
    )
}

pub(crate) unsafe extern "C" fn userdata_metamethod_callback(
    builder: *mut IrBuilder,
    lhs_ty: u8,
    rhs_ty: u8,
    result_reg: i32,
    lhs: IrOp,
    rhs: IrOp,
    method: HostMetamethod,
    pcpos: i32,
) -> bool {
    userdata_metamethod(
        &mut *builder,
        lhs_ty,
        rhs_ty,
        result_reg,
        lhs,
        rhs,
        method,
        pcpos,
    )
}

pub(crate) unsafe extern "C" fn userdata_namecall_callback(
    builder: *mut IrBuilder,
    r#type: u8,
    member: *const c_char,
    member_length: usize,
    arg_res_reg: i32,
    source_reg: i32,
    params: i32,
    results: i32,
    pcpos: i32,
) -> bool {
    userdata_namecall(
        &mut *builder,
        r#type,
        member,
        member_length,
        arg_res_reg,
        source_reg,
        params,
        results,
        pcpos,
    )
}

unsafe extern "C" fn userdata_remapper(
    _context: *mut c_void,
    name: *const c_char,
    name_length: usize,
) -> u8 {
    if compare_member_name(name, name_length, c"extra".as_ptr()) {
        return 0;
    }
    if compare_member_name(name, name_length, c"color".as_ptr()) {
        return 1;
    }
    if compare_member_name(name, name_length, c"vec2".as_ptr()) {
        return 2;
    }
    if compare_member_name(name, name_length, c"mat3".as_ptr()) {
        return 3;
    }
    if compare_member_name(name, name_length, c"vertex".as_ptr()) {
        return 4;
    }

    0xff
}

impl LoweringFixture {
    pub fn initialize_codegen(&mut self, l: *mut lua_State) {
        if luau_codegen_supported() != 0 {
            luau_codegen_create(l);
            set_userdata_remapper(l, core::ptr::null_mut(), userdata_remapper);
        }
    }
}
