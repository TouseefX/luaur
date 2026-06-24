use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;
use crate::records::scoped_spills::ScopedSpills;
use luaur_vm::records::global_state::global_State;
use luaur_vm::records::lua_state::lua_State;

pub fn call_step_gc(regs: &mut IrRegAllocX64, build: &mut AssemblyBuilderX64) {
    let mut skip = Label::default();

    {
        let mut tmp1 = ScopedRegX64 {
            owner: core::ptr::null_mut(),
            reg: RegisterX64::noreg,
        };
        tmp1.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::qword);

        let mut tmp2 = ScopedRegX64 {
            owner: core::ptr::null_mut(),
            reg: RegisterX64::noreg,
        };
        tmp2.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::qword);

        build.mov(
            OperandX64::reg(tmp1.reg),
            mem(
                SizeX64::qword,
                r_state(),
                core::mem::offset_of!(lua_State, global) as i32,
            ),
        );
        build.mov(
            OperandX64::reg(tmp2.reg),
            mem(
                SizeX64::qword,
                tmp1.reg,
                core::mem::offset_of!(global_State, totalbytes) as i32,
            ),
        );
        build.cmp(
            OperandX64::reg(tmp2.reg),
            mem(
                SizeX64::qword,
                tmp1.reg,
                core::mem::offset_of!(global_State, GCthreshold) as i32,
            ),
        );
        build.jcc(ConditionX64::Below, &mut skip);
    }

    {
        let _spill_guard = {
            let mut guard = ScopedSpills {
                owner: regs as *mut _,
                start_spill_id: 0,
            };
            guard.scoped_spills_scoped_spills_ir_reg_alloc_x_64(regs);
            guard
        };

        let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
            regs,
            build,
            k_invalid_inst_idx,
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(r_state()),
            IrOp::ir_op(),
        );
        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::dword,
            OperandX64::imm(1),
            IrOp::ir_op(),
        );
        call_wrap.call(&mem(
            SizeX64::qword,
            r_native_context(),
            core::mem::offset_of!(NativeContext, luaC_step) as i32,
        ));
        emit_update_base(build);
    }

    build.set_label_label(&mut skip);
}

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
    RegisterX64 {
        bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
    }
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
