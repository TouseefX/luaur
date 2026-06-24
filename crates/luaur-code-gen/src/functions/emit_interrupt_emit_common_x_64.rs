use crate::enums::abix_64::ABIX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::byte_reg::byte_reg;
use crate::functions::dword_reg::dword_reg;
use crate::functions::emit_exit_emit_common_x_64::emit_exit;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::label::Label;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::global_state::global_State;
use luaur_vm::records::lua_callbacks::LuaCallbacks;
use luaur_vm::records::lua_state::lua_State;

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

// C++ EmitCommonX64.h: `sCode = qword[rsp + kStackOffsetToLocals + 8]`
const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_code() -> OperandX64 {
    OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::rsp,
        K_STACK_OFFSET_TO_LOCALS + 8,
    )
}

const SIZEOF_INSTRUCTION: i32 = 4;

pub fn emit_interrupt(build: &mut AssemblyBuilderX64) {
    // rax = pcpos + 1
    // rbx = return address in native code
    // note: rbx is non-volatile so it will be saved across interrupt call automatically

    let r_arg1: RegisterX64;
    let r_arg2: RegisterX64;
    if luaur_common::FFlag::LuauCodegenSuggestArgumentRegisterX64.get() {
        r_arg1 = IrCallWrapperX64::suggest_argument_register::<0>(SizeX64::qword, build);
        r_arg2 = IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::qword, build);
    } else {
        r_arg1 = if build.abi == ABIX64::Windows {
            RegisterX64::rcx
        } else {
            RegisterX64::rdi
        };
        r_arg2 = if build.abi == ABIX64::Windows {
            RegisterX64::rdx
        } else {
            RegisterX64::rsi
        };
    }

    let mut skip = Label { id: 0, location: 0 };

    // Update L->ci->savedpc; required in case interrupt errors
    build.mov(OperandX64::reg(RegisterX64::rcx), s_code());
    build.lea_operand_x_64_operand_x_64(
        OperandX64::reg(RegisterX64::rcx),
        OperandX64::mem(
            SizeX64::none,
            RegisterX64::rax,
            SIZEOF_INSTRUCTION as u8,
            RegisterX64::rcx,
            0,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, ci) as i32,
        ),
    );
    build.mov(
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            RegisterX64::rax,
            core::mem::offset_of!(CallInfo, savedpc) as i32,
        ),
        OperandX64::reg(RegisterX64::rcx),
    );

    // Load interrupt handler; it may be nullptr in case the update raced with the check before we got here
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, global) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            RegisterX64::rax,
            (core::mem::offset_of!(global_State, cb)
                + core::mem::offset_of!(LuaCallbacks, interrupt)) as i32,
        ),
    );
    build.test(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::reg(RegisterX64::rax),
    );
    build.jcc(ConditionX64::Zero, &mut skip);

    // Call interrupt
    build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
    build.mov(OperandX64::reg(dword_reg(r_arg2)), OperandX64::imm(-1));
    build.call_operand_x_64(OperandX64::reg(RegisterX64::rax));

    // Check if we need to exit
    build.mov(
        OperandX64::reg(byte_reg(RegisterX64::rax)),
        OperandX64::mem(
            SizeX64::byte,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, status) as i32,
        ),
    );
    build.test(
        OperandX64::reg(byte_reg(RegisterX64::rax)),
        OperandX64::reg(byte_reg(RegisterX64::rax)),
    );
    build.jcc(ConditionX64::Zero, &mut skip);

    build.mov(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, ci) as i32,
        ),
    );
    build.sub(
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            RegisterX64::rax,
            core::mem::offset_of!(CallInfo, savedpc) as i32,
        ),
        OperandX64::imm(SIZEOF_INSTRUCTION),
    );
    emit_exit(build, /* continue_in_vm */ false);

    build.set_label_label(&mut skip);

    emit_update_base(build); // interrupt may have reallocated stack

    build.jmp_operand_x_64(OperandX64::reg(RegisterX64::rbx));
}
