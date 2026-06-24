use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::condition_a_64::ConditionA64;
use crate::enums::kind_a_64::KindA64;
use crate::functions::countrz_bit_utils::countrz_u32;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::module_helpers::ModuleHelpers;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;
use luaur_vm::macros::lua_callinfo_return::LUA_CALLINFO_RETURN;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::t_value::TValue;
use luaur_vm::type_aliases::value::Value;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const X0: RegisterA64 = reg(KindA64::x, 0);
const X1: RegisterA64 = reg(KindA64::x, 1);
const X2: RegisterA64 = reg(KindA64::x, 2);
const X3: RegisterA64 = reg(KindA64::x, 3);
const X4: RegisterA64 = reg(KindA64::x, 4);
const W2: RegisterA64 = reg(KindA64::w, 2);
const W3: RegisterA64 = reg(KindA64::w, 3);
const W4: RegisterA64 = reg(KindA64::w, 4);
const R_STATE: RegisterA64 = reg(KindA64::x, 19);
const R_BASE: RegisterA64 = reg(KindA64::x, 22);
const R_CONSTANTS: RegisterA64 = reg(KindA64::x, 23);
const R_CODE: RegisterA64 = reg(KindA64::x, 24);
const R_CLOSURE: RegisterA64 = reg(KindA64::x, 25);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

fn mem_reg(base: RegisterA64, offset: RegisterA64) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::reg,
        base,
        offset,
        data: 0,
    }
}

pub fn emit_return_assembly_builder_a_64_module_helpers(
    build: &mut AssemblyBuilderA64,
    helpers: &mut ModuleHelpers,
) {
    build.ldr(
        X0,
        mem(R_STATE, core::mem::offset_of!(lua_State, ci) as i32),
    );
    build.ldr(
        W3,
        mem(X0, core::mem::offset_of!(CallInfo, nresults) as i32),
    );

    let mut skip_result_copy = Label::default();

    build.cmp_register_a_64_register_a_64(W2, W3);
    build.b_condition_a_64_label(ConditionA64::GreaterEqual, &mut skip_result_copy);

    build.sub_register_a_64_register_a_64_register_a_64_i32(W2, W3, W2, 0);
    build.mov_register_a_64_i32(W4, lua_Type::LUA_TNIL as i32);

    let mut repeat_nil_loop = build.set_label();
    build.str(W4, mem(X1, core::mem::offset_of!(TValue, tt) as i32));
    build.add_register_a_64_register_a_64_u16(X1, X1, core::mem::size_of::<TValue>() as u16);
    build.sub_register_a_64_register_a_64_u16(W2, W2, 1);
    build.cbnz(W2, &mut repeat_nil_loop);

    build.set_label_label(&mut skip_result_copy);

    build.sub_register_a_64_register_a_64_u16(X2, X0, core::mem::size_of::<CallInfo>() as u16);

    let mut skip_fixed_ret_top = Label::default();
    build.tbnz(W3, 31, &mut skip_fixed_ret_top);
    build.ldr(X1, mem(X2, core::mem::offset_of!(CallInfo, top) as i32));
    build.set_label_label(&mut skip_fixed_ret_top);

    build.str(
        X2,
        mem(R_STATE, core::mem::offset_of!(lua_State, ci) as i32),
    );
    build.ldr(
        R_BASE,
        mem(X2, core::mem::offset_of!(CallInfo, base) as i32),
    );
    build.str(
        R_BASE,
        mem(R_STATE, core::mem::offset_of!(lua_State, base) as i32),
    );

    build.str(
        X1,
        mem(R_STATE, core::mem::offset_of!(lua_State, top) as i32),
    );

    if luaur_common::FFlag::LuauClosureUsageCounter.get() {
        build.ldr(
            X4,
            mem(R_CLOSURE, core::mem::offset_of!(Closure, usage) as i32),
        );
        build.sub_register_a_64_register_a_64_u16(X4, X4, 1);
        build.str(
            X4,
            mem(R_CLOSURE, core::mem::offset_of!(Closure, usage) as i32),
        );
    }

    build.ldr(W4, mem(X0, core::mem::offset_of!(CallInfo, flags) as i32));
    build.tbnz(
        W4,
        countrz_u32(LUA_CALLINFO_RETURN as u32) as u8,
        &mut helpers.exitNoContinueVm,
    );

    build.ldr(W4, mem(X2, core::mem::offset_of!(CallInfo, flags) as i32));
    build.tbz(
        W4,
        countrz_u32(LUA_CALLINFO_NATIVE as u32) as u8,
        &mut helpers.exitContinueVm,
    );

    build.ldr(
        R_CLOSURE,
        mem(X2, core::mem::offset_of!(CallInfo, func) as i32),
    );
    build.ldr(
        R_CLOSURE,
        mem(
            R_CLOSURE,
            (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32,
        ),
    );

    build.ldr(
        X1,
        mem(
            R_CLOSURE,
            (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
        ),
    );

    if luaur_common::DFFlag::AddReturnExectargetCheck.get() {
        build.ldp(
            X3,
            X4,
            mem(X1, core::mem::offset_of!(Proto, execdata) as i32),
        );
        build.cbz(X4, &mut helpers.exitContinueVmClearNativeFlag);
    }

    build.ldp(
        R_CONSTANTS,
        R_CODE,
        mem(X1, core::mem::offset_of!(Proto, k) as i32),
    );

    build.ldr(X2, mem(X2, core::mem::offset_of!(CallInfo, savedpc) as i32));
    build.sub_register_a_64_register_a_64_register_a_64_i32(X2, X2, R_CODE, 0);

    if !luaur_common::DFFlag::AddReturnExectargetCheck.get() {
        build.ldp(
            X3,
            X4,
            mem(X1, core::mem::offset_of!(Proto, execdata) as i32),
        );
    }

    build.ldr(W2, mem_reg(X3, X2));
    build.add_register_a_64_register_a_64_register_a_64_i32(X4, X4, X2, 0);
    build.br(X4);
}
