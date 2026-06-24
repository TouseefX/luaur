use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::dword_reg::dword_reg;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;
use crate::records::module_helpers::ModuleHelpers;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;
use luaur_vm::macros::lua_callinfo_return::LUA_CALLINFO_RETURN;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::t_value::TValue;
use luaur_vm::type_aliases::value::Value;

const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
const fn r_constants() -> RegisterX64 {
    RegisterX64 {
        bits: (12u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_closure() -> OperandX64 {
    OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::rsp,
        K_STACK_OFFSET_TO_LOCALS,
    )
}

fn s_code() -> OperandX64 {
    OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::rsp,
        K_STACK_OFFSET_TO_LOCALS + 8,
    )
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

pub fn emit_return(build: &mut AssemblyBuilderX64, helpers: &mut ModuleHelpers) {
    // input: res in rdi, number of written values in ecx
    let res = RegisterX64::rdi;
    let written = dword_reg(RegisterX64::rcx);

    let ci = RegisterX64::r8;
    let cip = RegisterX64::r9;
    let nresults = dword_reg(RegisterX64::rsi);

    let tvalue_size = core::mem::size_of::<TValue>() as i32;
    let callinfo_size = core::mem::size_of::<CallInfo>() as i32;

    build.mov(
        OperandX64::reg(ci),
        mem(
            SizeX64::qword,
            r_state(),
            core::mem::offset_of!(luaur_vm::records::lua_state::lua_State, ci) as i32,
        ),
    );
    build.lea_operand_x_64_operand_x_64(
        OperandX64::reg(cip),
        mem(SizeX64::none, ci, -callinfo_size),
    );

    // nresults = ci->nresults
    build.mov(
        OperandX64::reg(nresults),
        mem(
            SizeX64::dword,
            ci,
            core::mem::offset_of!(CallInfo, nresults) as i32,
        ),
    );

    let mut skip_result_copy = Label { id: 0, location: 0 };

    // Fill the rest of the expected results (nresults - written) with 'nil'
    let counter = written;
    build.sub(OperandX64::reg(counter), OperandX64::reg(nresults)); // counter = -(nresults - written)
    build.jcc(ConditionX64::GreaterEqual, &mut skip_result_copy);

    let mut repeat_nil_loop = Label { id: 0, location: 0 };
    build.set_label(&mut repeat_nil_loop);
    build.mov(
        mem(
            SizeX64::dword,
            res,
            core::mem::offset_of!(TValue, tt) as i32,
        ),
        OperandX64::imm(lua_Type::LUA_TNIL as i32),
    );
    build.add(OperandX64::reg(res), OperandX64::imm(tvalue_size));
    build.inc(OperandX64::reg(counter));
    build.jcc(ConditionX64::NotZero, &mut repeat_nil_loop);

    build.set_label_label(&mut skip_result_copy);

    // L->ci = cip
    build.mov(
        mem(
            SizeX64::qword,
            r_state(),
            core::mem::offset_of!(luaur_vm::records::lua_state::lua_State, ci) as i32,
        ),
        OperandX64::reg(cip),
    );
    // sync base = L->base while we have a chance (rBase = rbp in this crate)
    build.mov(
        OperandX64::reg(RegisterX64::rbp),
        mem(
            SizeX64::qword,
            cip,
            core::mem::offset_of!(CallInfo, base) as i32,
        ),
    );
    // L->base = cip->base
    build.mov(
        mem(
            SizeX64::qword,
            r_state(),
            core::mem::offset_of!(luaur_vm::records::lua_state::lua_State, base) as i32,
        ),
        OperandX64::reg(RegisterX64::rbp),
    );

    let mut skip_fixed_ret_top = Label { id: 0, location: 0 };
    build.test(OperandX64::reg(nresults), OperandX64::reg(nresults));
    build.jcc(ConditionX64::Less, &mut skip_fixed_ret_top);
    build.mov(
        OperandX64::reg(res),
        mem(
            SizeX64::qword,
            cip,
            core::mem::offset_of!(CallInfo, top) as i32,
        ),
    );
    build.set_label_label(&mut skip_fixed_ret_top);

    // L->top = res
    build.mov(
        mem(
            SizeX64::qword,
            r_state(),
            core::mem::offset_of!(luaur_vm::records::lua_state::lua_State, top) as i32,
        ),
        OperandX64::reg(res),
    );

    if luaur_common::FFlag::LuauClosureUsageCounter.get() {
        build.mov(OperandX64::reg(RegisterX64::rax), s_closure());
        build.dec(mem(
            SizeX64::qword,
            RegisterX64::rax,
            core::mem::offset_of!(Closure, usage) as i32,
        ));
    }

    // Unlikely, but this might be the last return from VM
    build.test(
        mem(
            SizeX64::byte,
            ci,
            core::mem::offset_of!(CallInfo, flags) as i32,
        ),
        OperandX64::imm(LUA_CALLINFO_RETURN),
    );
    build.jcc(ConditionX64::NotZero, &mut helpers.exitNoContinueVm);

    // Returning back to the previous function is a bit tricky
    // Registers alive: r9 (cip)
    let proto = RegisterX64::rcx;
    let execdata = RegisterX64::rbx;
    let exectarget = RegisterX64::r10;

    // Change closure
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            cip,
            core::mem::offset_of!(CallInfo, func) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            RegisterX64::rax,
            (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32,
        ),
    );
    build.mov(s_closure(), OperandX64::reg(RegisterX64::rax));

    build.mov(
        OperandX64::reg(proto),
        mem(
            SizeX64::qword,
            RegisterX64::rax,
            (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
        ),
    );

    build.mov(
        OperandX64::reg(execdata),
        mem(
            SizeX64::qword,
            proto,
            core::mem::offset_of!(Proto, execdata) as i32,
        ),
    );

    build.test(
        mem(
            SizeX64::byte,
            cip,
            core::mem::offset_of!(CallInfo, flags) as i32,
        ),
        OperandX64::imm(LUA_CALLINFO_NATIVE),
    );
    build.jcc(ConditionX64::Zero, &mut helpers.exitContinueVm); // Continue in interpreter if function has no native data

    if luaur_common::DFFlag::AddReturnExectargetCheck.get() {
        build.mov(
            OperandX64::reg(exectarget),
            mem(
                SizeX64::qword,
                proto,
                core::mem::offset_of!(Proto, exectarget) as i32,
            ),
        );
        build.test(OperandX64::reg(exectarget), OperandX64::reg(exectarget));
        build.jcc(
            ConditionX64::Zero,
            &mut helpers.exitContinueVmClearNativeFlag,
        );
    }

    // Change constants
    build.mov(
        OperandX64::reg(r_constants()),
        mem(
            SizeX64::qword,
            proto,
            core::mem::offset_of!(Proto, k) as i32,
        ),
    );

    // Change code
    build.mov(
        OperandX64::reg(RegisterX64::rdx),
        mem(
            SizeX64::qword,
            proto,
            core::mem::offset_of!(Proto, code) as i32,
        ),
    );
    build.mov(s_code(), OperandX64::reg(RegisterX64::rdx));

    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            cip,
            core::mem::offset_of!(CallInfo, savedpc) as i32,
        ),
    );

    // To get instruction index from instruction pointer, we need to divide byte offset by 4
    // But we will actually need to scale instruction index by 4 back to byte offset later so it cancels out
    build.sub(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::reg(RegisterX64::rdx),
    );

    // Get new instruction location and jump to it
    build.mov(
        OperandX64::reg(dword_reg(RegisterX64::rdx)),
        OperandX64::mem(SizeX64::dword, RegisterX64::rax, 1, execdata, 0),
    );

    if luaur_common::DFFlag::AddReturnExectargetCheck.get() {
        build.add(
            OperandX64::reg(RegisterX64::rdx),
            OperandX64::reg(exectarget),
        );
    } else {
        build.add(
            OperandX64::reg(RegisterX64::rdx),
            mem(
                SizeX64::qword,
                proto,
                core::mem::offset_of!(Proto, exectarget) as i32,
            ),
        );
    }
    build.jmp_operand_x_64(OperandX64::reg(RegisterX64::rdx));
}
