use crate::enums::abix_64::ABIX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::call_barrier_table_fast::call_barrier_table_fast;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::lua_table::LuaTable;
use luaur_vm::type_aliases::t_value::TValue;

pub fn emit_inst_set_list(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    ra: i32,
    rb: i32,
    count: i32,
    index: u32,
    known_size: i32,
) {
    let mut last = OperandX64::imm(index as i32 + count - 1);
    let cscaled = RegisterX64::rbx;

    if count == LUA_MULTRET {
        let tmp = RegisterX64::rax;

        build.mov(
            OperandX64::reg(cscaled),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, top) as i32,
            ),
        );
        build.lea_operand_x_64_operand_x_64(OperandX64::reg(tmp), luau_reg_address(rb));
        build.sub(OperandX64::reg(cscaled), OperandX64::reg(tmp));

        build.mov(
            OperandX64::reg(tmp),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, ci) as i32,
            ),
        );
        build.mov(
            OperandX64::reg(tmp),
            mem(
                SizeX64::qword,
                tmp,
                core::mem::offset_of!(CallInfo, top) as i32,
            ),
        );
        build.mov(
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, top) as i32,
            ),
            OperandX64::reg(tmp),
        );

        last = OperandX64::reg(sized(RegisterX64::rdx, SizeX64::dword));
        build.mov(last, OperandX64::reg(sized(cscaled, SizeX64::dword)));
        build.shr(last, OperandX64::imm(K_TVALUE_SIZE_LOG2));
        build.add(last, OperandX64::imm(index as i32 - 1));
    }

    let mut table = regs.take_reg(RegisterX64::rax, k_invalid_inst_idx);
    build.mov(OperandX64::reg(table), luau_reg_value(ra));

    if count == LUA_MULTRET || known_size < 0 || known_size < (index as i32 + count - 1) {
        let mut skip_resize = Label::default();

        build.cmp(
            mem(
                SizeX64::dword,
                table,
                core::mem::offset_of!(LuaTable, sizearray) as i32,
            ),
            last,
        );
        build.jcc(ConditionX64::NotBelow, &mut skip_resize);

        if luaur_common::FFlag::LuauCodeGenCallWrapperEmitInst.get() {
            if count == LUA_MULTRET {
                regs.take_reg(last.base, k_invalid_inst_idx);
            }

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
                SizeX64::dword,
                last,
                IrOp::ir_op(),
            );
            call_wrapper
                .call(&native_context_slot(
                    core::mem::offset_of!(NativeContext, luaH_resizearray) as i32,
                ));

            table = regs.take_reg(RegisterX64::rax, k_invalid_inst_idx);
        } else {
            let (_, r_arg2, r_arg3, _) = abi_arg_regs(build);
            let r_arg1 = if build.abi == ABIX64::Windows {
                RegisterX64::rcx
            } else {
                RegisterX64::rdi
            };

            crate::macros::codegen_assert::CODEGEN_ASSERT!(r_arg3 != table);
            build.mov(OperandX64::reg(sized(r_arg3, SizeX64::dword)), last);
            build.mov(OperandX64::reg(r_arg2), OperandX64::reg(table));
            build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
            build.call_operand_x_64(native_context_slot(core::mem::offset_of!(
                NativeContext,
                luaH_resizearray
            ) as i32));
        }

        build.mov(OperandX64::reg(table), luau_reg_value(ra));
        build.set_label_label(&mut skip_resize);
    }

    let array_dst = RegisterX64::rdx;
    let offset = RegisterX64::rcx;

    build.mov(
        OperandX64::reg(array_dst),
        mem(
            SizeX64::qword,
            table,
            core::mem::offset_of!(LuaTable, array) as i32,
        ),
    );

    const K_UNROLL_SET_LIST_LIMIT: i32 = 4;

    if count != LUA_MULTRET && count <= K_UNROLL_SET_LIST_LIMIT {
        for i in 0..count {
            build.vmovups(OperandX64::reg(xmm(0)), luau_reg_value(rb + i));
            build.vmovups(
                mem(
                    SizeX64::xmmword,
                    array_dst,
                    (index as i32 + i - 1) * core::mem::size_of::<TValue>() as i32,
                ),
                OperandX64::reg(xmm(0)),
            );
        }
    } else {
        crate::macros::codegen_assert::CODEGEN_ASSERT!(count != 0);

        build.xor_(OperandX64::reg(offset), OperandX64::reg(offset));
        if index != 1 {
            build.add(
                OperandX64::reg(array_dst),
                OperandX64::imm((index as i32 - 1) * core::mem::size_of::<TValue>() as i32),
            );
        }

        let mut repeat_loop = Label::default();
        let mut end_loop = Label::default();
        let limit = if count == LUA_MULTRET {
            OperandX64::reg(cscaled)
        } else {
            OperandX64::imm(count * core::mem::size_of::<TValue>() as i32)
        };

        if count == LUA_MULTRET {
            build.cmp(OperandX64::reg(offset), limit);
            build.jcc(ConditionX64::NotBelow, &mut end_loop);
        }

        build.set_label(&mut repeat_loop);
        build.vmovups(
            OperandX64::reg(xmm(0)),
            OperandX64::mem(
                SizeX64::xmmword,
                offset,
                1,
                r_base(),
                rb * core::mem::size_of::<TValue>() as i32,
            ),
        );
        build.vmovups(
            OperandX64::mem(SizeX64::xmmword, offset, 1, array_dst, 0),
            OperandX64::reg(xmm(0)),
        );

        build.add(
            OperandX64::reg(offset),
            OperandX64::imm(core::mem::size_of::<TValue>() as i32),
        );
        build.cmp(OperandX64::reg(offset), limit);
        build.jcc(ConditionX64::Below, &mut repeat_loop);

        build.set_label_label(&mut end_loop);
    }

    call_barrier_table_fast(regs, build, table, IrOp::ir_op());
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

const fn r_base() -> RegisterX64 {
    RegisterX64::rbp
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

fn native_context_slot(disp: i32) -> OperandX64 {
    mem(SizeX64::qword, r_native_context(), disp)
}

fn luau_reg_address(ri: i32) -> OperandX64 {
    crate::functions::luau_reg_address::luau_reg_address(ri)
}

fn abi_arg_regs(
    build: &AssemblyBuilderX64,
) -> (RegisterX64, RegisterX64, RegisterX64, RegisterX64) {
    if build.abi == ABIX64::Windows {
        (
            RegisterX64::rcx,
            RegisterX64::rdx,
            RegisterX64::r8,
            RegisterX64::r9,
        )
    } else {
        (
            RegisterX64::rdi,
            RegisterX64::rsi,
            RegisterX64::rdx,
            RegisterX64::rcx,
        )
    }
}
