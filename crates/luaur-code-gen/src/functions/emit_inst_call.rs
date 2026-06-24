use crate::enums::abix_64::ABIX64;
use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::dword_reg::dword_reg;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::functions::get_full_stack_size::kStackOffsetToLocals;
use crate::functions::luau_reg::luau_reg;
use crate::functions::luau_reg_address::luau_reg_address;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::label::Label;
use crate::records::module_helpers::ModuleHelpers;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::closure::{CClosure, Closure, LClosure};
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::t_value::TValue;

pub fn emit_inst_call(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    helpers: &mut ModuleHelpers,
    ra: i32,
    nparams: i32,
    nresults: i32,
) {
    if luaur_common::FFlag::LuauCodeGenCallWrapperEmitInst.get() {
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
            luau_reg_address(ra),
            IrOp::ir_op(),
        );
        if nparams == LUA_MULTRET {
            call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::qword,
                mem(
                    SizeX64::qword,
                    r_state(),
                    core::mem::offset_of!(lua_State, top) as i32,
                ),
                IrOp::ir_op(),
            );
        } else {
            call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
                SizeX64::qword,
                luau_reg_address(ra + 1 + nparams),
                IrOp::ir_op(),
            );
        }
        call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::dword,
            OperandX64::imm(nresults),
            IrOp::ir_op(),
        );
        call_wrapper.call(&native_context_slot(
            core::mem::offset_of!(NativeContext, callProlog) as i32,
        ));
    } else {
        let (r_arg1, r_arg2, r_arg3, r_arg4) = abi_arg_regs(build);

        build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
        build.lea_operand_x_64_operand_x_64(OperandX64::reg(r_arg2), luau_reg_address(ra));

        if nparams == LUA_MULTRET {
            build.mov(
                OperandX64::reg(r_arg3),
                mem(
                    SizeX64::qword,
                    r_state(),
                    core::mem::offset_of!(lua_State, top) as i32,
                ),
            );
        } else {
            build.lea_operand_x_64_operand_x_64(
                OperandX64::reg(r_arg3),
                luau_reg_address(ra + 1 + nparams),
            );
        }

        build.mov(
            OperandX64::reg(sized(r_arg4, SizeX64::dword)),
            OperandX64::imm(nresults),
        );
        build.call_operand_x_64(native_context_slot(
            core::mem::offset_of!(NativeContext, callProlog) as i32,
        ));
    }

    let ccl = RegisterX64::rax;
    emit_update_base(build);

    let mut c_func_call = Label::default();

    build.test(
        mem(
            SizeX64::byte,
            ccl,
            core::mem::offset_of!(Closure, isC) as i32,
        ),
        OperandX64::imm(1),
    );
    build.jcc(ConditionX64::NotZero, &mut c_func_call);

    {
        let proto = RegisterX64::rcx;
        let ci = RegisterX64::rdx;
        let argi = RegisterX64::rsi;
        let argend = RegisterX64::rdi;

        build.mov(
            OperandX64::reg(proto),
            mem(
                SizeX64::qword,
                ccl,
                (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
            ),
        );

        build.mov(s_closure(), OperandX64::reg(ccl));
        build.mov(
            OperandX64::reg(ci),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, ci) as i32,
            ),
        );

        let mut fillnil = Label::default();
        let mut exitfillnil = Label::default();

        build.mov(
            OperandX64::reg(argi),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, top) as i32,
            ),
        );

        build.movzx(
            sized(RegisterX64::rax, SizeX64::dword),
            mem(
                SizeX64::byte,
                proto,
                core::mem::offset_of!(Proto, numparams) as i32,
            ),
        );
        build.shl(
            OperandX64::reg(sized(RegisterX64::rax, SizeX64::dword)),
            OperandX64::imm(K_TVALUE_SIZE_LOG2),
        );
        build.lea_operand_x_64_operand_x_64(
            OperandX64::reg(argend),
            OperandX64::mem(
                SizeX64::none,
                sized(RegisterX64::rax, SizeX64::qword),
                1,
                r_base(),
                0,
            ),
        );

        build.set_label(&mut fillnil);
        build.cmp(OperandX64::reg(argi), OperandX64::reg(argend));
        build.jcc(ConditionX64::NotBelow, &mut exitfillnil);

        build.mov(
            mem(
                SizeX64::dword,
                argi,
                core::mem::offset_of!(TValue, tt) as i32,
            ),
            OperandX64::imm(lua_Type::LUA_TNIL as i32),
        );
        build.add(
            OperandX64::reg(argi),
            OperandX64::imm(core::mem::size_of::<TValue>() as i32),
        );
        build.jmp_label(&mut fillnil);

        build.set_label_label(&mut exitfillnil);

        build.mov(
            OperandX64::reg(RegisterX64::rax),
            mem(
                SizeX64::qword,
                ci,
                core::mem::offset_of!(CallInfo, top) as i32,
            ),
        );

        let mut skip_vararg = Label::default();
        build.test(
            mem(
                SizeX64::byte,
                proto,
                core::mem::offset_of!(Proto, is_vararg) as i32,
            ),
            OperandX64::imm(1),
        );
        build.jcc(ConditionX64::Zero, &mut skip_vararg);
        build.mov(OperandX64::reg(RegisterX64::rax), OperandX64::reg(argi));

        build.set_label_label(&mut skip_vararg);

        build.mov(
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, top) as i32,
            ),
            OperandX64::reg(RegisterX64::rax),
        );

        build.mov(
            OperandX64::reg(RegisterX64::rax),
            mem(
                SizeX64::qword,
                proto,
                core::mem::offset_of!(Proto, code) as i32,
            ),
        );
        build.mov(s_code(), OperandX64::reg(RegisterX64::rax));
        build.mov(
            mem(
                SizeX64::qword,
                ci,
                core::mem::offset_of!(CallInfo, savedpc) as i32,
            ),
            OperandX64::reg(RegisterX64::rax),
        );

        build.mov(
            OperandX64::reg(r_constants()),
            mem(
                SizeX64::qword,
                proto,
                core::mem::offset_of!(Proto, k) as i32,
            ),
        );

        build.mov(
            OperandX64::reg(RegisterX64::rax),
            mem(
                SizeX64::qword,
                proto,
                core::mem::offset_of!(Proto, exectarget) as i32,
            ),
        );
        build.test(
            OperandX64::reg(RegisterX64::rax),
            OperandX64::reg(RegisterX64::rax),
        );
        build.jcc(ConditionX64::Zero, &mut helpers.exitContinueVm);

        build.mov(
            mem(
                SizeX64::dword,
                ci,
                core::mem::offset_of!(CallInfo, flags) as i32,
            ),
            OperandX64::imm(LUA_CALLINFO_NATIVE as i32),
        );

        build.jmp_operand_x_64(OperandX64::reg(RegisterX64::rax));
    }

    build.set_label_label(&mut c_func_call);

    {
        if luaur_common::FFlag::LuauCodeGenCallWrapperEmitInst.get() {
            regs.take_reg(ccl, k_invalid_inst_idx);
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
            call_wrapper.call(&mem(
                SizeX64::qword,
                ccl,
                (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(CClosure, f)) as i32,
            ));
        } else {
            let (r_arg1, _, _, _) = abi_arg_regs(build);
            build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
            build.call_operand_x_64(mem(
                SizeX64::qword,
                ccl,
                (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(CClosure, f)) as i32,
            ));
        }

        let results = sized(RegisterX64::rax, SizeX64::dword);

        build.test(OperandX64::reg(results), OperandX64::reg(results));
        build.jcc(ConditionX64::Less, &mut helpers.exitNoContinueVm);

        if nresults != 0 && nresults != 1 {
            if luaur_common::FFlag::LuauCodeGenCallWrapperEmitInst.get() {
                regs.take_reg(results, k_invalid_inst_idx);
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
                    SizeX64::dword,
                    OperandX64::imm(nresults),
                    IrOp::ir_op(),
                );
                call_wrapper.add_argument_size_x_64_operand_x_64_ir_op(
                    SizeX64::dword,
                    OperandX64::reg(results),
                    IrOp::ir_op(),
                );
                call_wrapper
                    .call(&native_context_slot(
                        core::mem::offset_of!(NativeContext, callEpilogC) as i32,
                    ));
            } else {
                let (r_arg1, r_arg2, r_arg3, _) = abi_arg_regs(build);

                build.mov(OperandX64::reg(r_arg1), OperandX64::reg(r_state()));
                build.mov(
                    OperandX64::reg(sized(r_arg2, SizeX64::dword)),
                    OperandX64::imm(nresults),
                );
                build.mov(
                    OperandX64::reg(sized(r_arg3, SizeX64::dword)),
                    OperandX64::reg(results),
                );
                build.call_operand_x_64(native_context_slot(core::mem::offset_of!(
                    NativeContext,
                    callEpilogC
                ) as i32));
            }

            emit_update_base(build);
            return;
        }

        let ci = RegisterX64::rdx;
        let cip = RegisterX64::rcx;
        let vali = RegisterX64::rsi;

        build.mov(
            OperandX64::reg(ci),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, ci) as i32,
            ),
        );
        build.lea_operand_x_64_operand_x_64(
            OperandX64::reg(cip),
            mem(
                SizeX64::none,
                ci,
                -(core::mem::size_of::<CallInfo>() as i32),
            ),
        );

        build.mov(
            OperandX64::reg(r_base()),
            mem(
                SizeX64::qword,
                cip,
                core::mem::offset_of!(CallInfo, base) as i32,
            ),
        );
        build.mov(
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, base) as i32,
            ),
            OperandX64::reg(r_base()),
        );

        if nresults == 1 {
            build.mov(
                OperandX64::reg(vali),
                mem(
                    SizeX64::qword,
                    r_state(),
                    core::mem::offset_of!(lua_State, top) as i32,
                ),
            );
            build.shl(
                OperandX64::reg(results),
                OperandX64::imm(K_TVALUE_SIZE_LOG2),
            );
            build.sub(
                OperandX64::reg(vali),
                OperandX64::reg(sized(results, SizeX64::qword)),
            );
            build.vmovups(
                OperandX64::reg(xmm(0)),
                OperandX64::mem(SizeX64::xmmword, RegisterX64::noreg, 1, vali, 0),
            );
            build.vmovups(luau_reg(ra), OperandX64::reg(xmm(0)));

            let mut skipnil = Label::default();
            build.test(OperandX64::reg(results), OperandX64::reg(results));
            build.jcc(ConditionX64::NotZero, &mut skipnil);
            build.mov(luau_reg_tag(ra), OperandX64::imm(lua_Type::LUA_TNIL as i32));
            build.set_label_label(&mut skipnil);
        }

        build.mov(
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, ci) as i32,
            ),
            OperandX64::reg(cip),
        );
        build.mov(
            OperandX64::reg(RegisterX64::rax),
            mem(
                SizeX64::qword,
                cip,
                core::mem::offset_of!(CallInfo, top) as i32,
            ),
        );
        build.mov(
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, top) as i32,
            ),
            OperandX64::reg(RegisterX64::rax),
        );
    }
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

const fn r_constants() -> RegisterX64 {
    reg(12, SizeX64::qword)
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

fn s_closure() -> OperandX64 {
    mem(
        SizeX64::qword,
        RegisterX64::rsp,
        kStackOffsetToLocals as i32,
    )
}

fn s_code() -> OperandX64 {
    mem(
        SizeX64::qword,
        RegisterX64::rsp,
        kStackOffsetToLocals as i32 + 8,
    )
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
