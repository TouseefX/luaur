use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::label::Label;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;
use crate::records::scoped_spills::ScopedSpills;

use crate::enums::size_x_64::SizeX64;

use crate::functions::check_object_barrier_conditions::check_object_barrier_conditions;

const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

const fn r_native_context() -> RegisterX64 {
    RegisterX64 {
        bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

pub fn call_barrier_object(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    object: RegisterX64,
    object_op: IrOp,
    ra: RegisterX64,
    ra_op: IrOp,
    ratag: i32,
) {
    let mut skip = Label { id: 0, location: 0 };

    let mut tmp = ScopedRegX64 {
        owner: regs as *mut _,
        reg: RegisterX64::noreg,
    };
    tmp.scoped_reg_x_64_ir_reg_alloc_x_64_size_x_64(regs, SizeX64::qword);

    check_object_barrier_conditions(build, tmp.reg, object, ra, ra_op, ratag, &mut skip);

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
            regs.curr_inst_idx,
        );

        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(r_state()),
            IrOp::ir_op(),
        );

        call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
            SizeX64::qword,
            OperandX64::reg(object),
            object_op,
        );

        call_wrap.add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp);

        let barrierf_offset = core::mem::offset_of!(NativeContext, luaC_barrierf) as i32;
        let target = OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_native_context(),
            barrierf_offset,
        );

        call_wrap.call(&target);
    }

    build.set_label_label(&mut skip);
}
