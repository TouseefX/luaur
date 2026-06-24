use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::functions::emit_invoke_libm_1_p::emit_invoke_libm_1_p;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::ir_function::IrFunction;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::native_context::NativeContext;
use crate::records::register_a_64::RegisterA64;
use luaur_common::enums::luau_builtin_function::LuauBuiltinFunction;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::t_value::TValue;

pub fn emit_builtin_assembly_builder_a_64_ir_function_ir_reg_alloc_a_64_i32_i32_i32_i32(
    build: &mut AssemblyBuilderA64,
    _function: &mut IrFunction,
    regs: &mut IrRegAllocA64,
    bfid: i32,
    res: i32,
    arg: i32,
    nresults: i32,
) -> bool {
    match bfid {
        x if x == LuauBuiltinFunction::LBF_MATH_FREXP as i32 => {
            crate::macros::codegen_assert::CODEGEN_ASSERT!(nresults == 1 || nresults == 2);
            emit_invoke_libm_1_p(build, core::mem::offset_of!(NativeContext, libm_frexp), arg);
            build.str(
                D0,
                tvalue_mem(res, core::mem::offset_of!(TValue, value) as i32),
            );

            let temp = regs.alloc_temp(KindA64::w);
            build.mov_register_a_64_i32(temp, lua_Type::LUA_TNUMBER as i32);
            build.str(
                temp,
                tvalue_mem(res, core::mem::offset_of!(TValue, tt) as i32),
            );

            if nresults == 2 {
                build.ldr(W0, s_temporary());
                build.scvtf(D1, W0);
                build.str(
                    D1,
                    tvalue_mem(res + 1, core::mem::offset_of!(TValue, value) as i32),
                );
                build.str(
                    temp,
                    tvalue_mem(res + 1, core::mem::offset_of!(TValue, tt) as i32),
                );
            }

            true
        }
        x if x == LuauBuiltinFunction::LBF_MATH_MODF as i32 => {
            crate::macros::codegen_assert::CODEGEN_ASSERT!(nresults == 1 || nresults == 2);
            emit_invoke_libm_1_p(build, core::mem::offset_of!(NativeContext, libm_modf), arg);
            build.ldr(D1, s_temporary());
            build.str(
                D1,
                tvalue_mem(res, core::mem::offset_of!(TValue, value) as i32),
            );

            let temp = regs.alloc_temp(KindA64::w);
            build.mov_register_a_64_i32(temp, lua_Type::LUA_TNUMBER as i32);
            build.str(
                temp,
                tvalue_mem(res, core::mem::offset_of!(TValue, tt) as i32),
            );

            if nresults == 2 {
                build.str(
                    D0,
                    tvalue_mem(res + 1, core::mem::offset_of!(TValue, value) as i32),
                );
                build.str(
                    temp,
                    tvalue_mem(res + 1, core::mem::offset_of!(TValue, tt) as i32),
                );
            }

            true
        }
        _ => {
            crate::macros::codegen_assert::CODEGEN_ASSERT!(false);
            false
        }
    }
}

const S_TEMPORARY_DATA: i32 = 9 * 8;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const W0: RegisterA64 = reg(KindA64::w, 0);
const X25: RegisterA64 = reg(KindA64::x, 25);
const SP: RegisterA64 = reg(KindA64::none, 31);
const D0: RegisterA64 = reg(KindA64::d, 0);
const D1: RegisterA64 = reg(KindA64::d, 1);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

fn tvalue_mem(reg: i32, offset: i32) -> AddressA64 {
    mem(X25, reg * core::mem::size_of::<TValue>() as i32 + offset)
}

fn s_temporary() -> AddressA64 {
    mem(SP, S_TEMPORARY_DATA)
}
