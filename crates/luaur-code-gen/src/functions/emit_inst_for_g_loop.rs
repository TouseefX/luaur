use crate::enums::abix_64::ABIX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::luau_reg::luau_reg;
use crate::functions::luau_reg_address::luau_reg_address;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::functions::set_luau_reg::set_luau_reg;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::records::lua_table::LuaTable;
use luaur_vm::type_aliases::t_value::TValue;

pub fn emit_inst_for_g_loop(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    ra: i32,
    aux: i32,
    loop_repeat: &mut Label,
) {
    crate::macros::codegen_assert::CODEGEN_ASSERT!(aux >= 0);

    let (table, index) = if luaur_common::FFlag::LuauCodegenSuggestArgumentRegisterX64.get() {
        (
            IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::qword, build),
            IrCallWrapperX64::suggest_argument_register::<2>(SizeX64::qword, build),
        )
    } else if build.abi == ABIX64::Windows {
        (RegisterX64::rdx, RegisterX64::r8)
    } else {
        (RegisterX64::rsi, RegisterX64::rdx)
    };

    let elem_ptr = RegisterX64::rax;

    build.mov(OperandX64::reg(table), luau_reg_value(ra + 1));
    build.mov(OperandX64::reg(index), luau_reg_value(ra + 2));

    build.mov(
        OperandX64::reg(sized(elem_ptr, SizeX64::dword)),
        OperandX64::reg(sized(index, SizeX64::dword)),
    );
    build.shl(
        OperandX64::reg(sized(elem_ptr, SizeX64::dword)),
        OperandX64::imm(K_TVALUE_SIZE_LOG2),
    );
    build.add(
        OperandX64::reg(elem_ptr),
        mem(
            SizeX64::qword,
            table,
            core::mem::offset_of!(LuaTable, array) as i32,
        ),
    );

    for i in 2..aux {
        build.mov(
            luau_reg_tag(ra + 3 + i),
            OperandX64::imm(lua_Type::LUA_TNIL as i32),
        );
    }

    let mut skip_array = Label::default();
    let mut skip_array_nil = Label::default();

    let mut array_loop = Label::default();
    build.set_label(&mut array_loop);
    build.cmp(
        OperandX64::reg(sized(index, SizeX64::dword)),
        mem(
            SizeX64::dword,
            table,
            core::mem::offset_of!(LuaTable, sizearray) as i32,
        ),
    );
    build.jcc(ConditionX64::NotBelow, &mut skip_array);

    build.inc(OperandX64::reg(index));

    build.cmp(
        mem(
            SizeX64::dword,
            elem_ptr,
            core::mem::offset_of!(TValue, tt) as i32,
        ),
        OperandX64::imm(lua_Type::LUA_TNIL as i32),
    );
    build.jcc(ConditionX64::Equal, &mut skip_array_nil);

    build.mov(luau_reg_value(ra + 2), OperandX64::reg(index));

    build.vcvtsi2sd(
        OperandX64::reg(xmm(0)),
        OperandX64::reg(xmm(0)),
        OperandX64::reg(sized(index, SizeX64::dword)),
    );
    build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra + 3), OperandX64::reg(xmm(0)));
    build.mov(
        luau_reg_tag(ra + 3),
        OperandX64::imm(lua_Type::LUA_TNUMBER as i32),
    );

    set_luau_reg(build, xmm(2), ra + 4, mem(SizeX64::xmmword, elem_ptr, 0));

    build.jmp_label(loop_repeat);

    build.set_label_label(&mut skip_array_nil);
    build.add(
        OperandX64::reg(elem_ptr),
        OperandX64::imm(core::mem::size_of::<TValue>() as i32),
    );
    build.jmp_label(&mut array_loop);

    build.set_label_label(&mut skip_array);

    if luaur_common::FFlag::LuauCodeGenCallWrapperEmitInst.get() {
        regs.take_reg(table, k_invalid_inst_idx);
        regs.take_reg(index, k_invalid_inst_idx);

        let mut call_wrapper = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            regs,
            build,
            k_invalid_inst_idx,
        );
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(r_state()),
            IrOp::ir_op(),
        );
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(table),
            IrOp::ir_op(),
        );
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(index),
            IrOp::ir_op(),
        );
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            luau_reg_address(ra),
            IrOp::ir_op(),
        );
        call_wrapper
            .call(&native_context_slot(
                core::mem::offset_of!(NativeContext, forgLoopNodeIter) as i32,
            ));
    } else {
        let r_arg1 = if build.abi == ABIX64::Windows {
            RegisterX64::rcx
        } else {
            RegisterX64::rdi
        };
        let r_arg4 = if build.abi == ABIX64::Windows {
            RegisterX64::r9
        } else {
            RegisterX64::rcx
        };

        build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
        build.lea_operand_x_64_operand_x_64(OperandX64::reg(r_arg4), luau_reg_address(ra));
        build.call_operand_x_64(native_context_slot(core::mem::offset_of!(
            NativeContext,
            forgLoopNodeIter
        ) as i32));
    }

    build.test(
        OperandX64::reg(sized(RegisterX64::rax, SizeX64::byte)),
        OperandX64::reg(sized(RegisterX64::rax, SizeX64::byte)),
    );
    build.jcc(ConditionX64::NotZero, loop_repeat);
}

const K_TVALUE_SIZE_LOG2: i32 = 4;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
    RegisterX64 {
        bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
    }
}

const fn sized(reg: RegisterX64, size: SizeX64) -> RegisterX64 {
    RegisterX64 {
        bits: (reg.index() << RegisterX64::INDEX_SHIFT) | size as u8,
    }
}

const fn xmm(index: u8) -> RegisterX64 {
    reg(index, SizeX64::xmmword)
}

const fn r_state() -> RegisterX64 {
    reg(15, SizeX64::qword)
}

const fn r_native_context() -> RegisterX64 {
    reg(13, SizeX64::qword)
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

fn native_context_slot(disp: i32) -> OperandX64 {
    mem(SizeX64::qword, r_native_context(), disp)
}
