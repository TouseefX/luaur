use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::macros::op_c::op_c;
use crate::macros::op_d::op_d;
use crate::macros::op_e::op_e;
use crate::macros::op_f::op_f;
use crate::macros::op_g::op_g;
use crate::records::block_vm_reg_live_in_computation::BlockVmRegLiveInComputation;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;

pub fn visit_vm_reg_defs_uses_t_ir_function_ir_inst(
    visitor: &mut BlockVmRegLiveInComputation<'_>,
    function: &mut IrFunction,
    inst: &mut IrInst,
) {
    match unsafe { (*inst).cmd } {
        IrCmd::LOAD_TAG
        | IrCmd::LOAD_POINTER
        | IrCmd::LOAD_DOUBLE
        | IrCmd::LOAD_INT
        | IrCmd::LOAD_INT64
        | IrCmd::LOAD_FLOAT
        | IrCmd::LOAD_TVALUE => {
            visitor.maybe_use(op_a(inst));
        }

        IrCmd::STORE_TAG
        | IrCmd::STORE_EXTRA
        | IrCmd::STORE_POINTER
        | IrCmd::STORE_DOUBLE
        | IrCmd::STORE_INT
        | IrCmd::STORE_INT64
        | IrCmd::STORE_VECTOR
        | IrCmd::STORE_TVALUE
        | IrCmd::STORE_SPLIT_TVALUE => {
            visitor.maybe_def(op_a(inst));
        }

        IrCmd::CMP_ANY => {
            visitor.r#use(op_a(inst), 0);
            visitor.r#use(op_b(inst.clone()), 0);
        }

        IrCmd::CMP_TAG => {
            visitor.maybe_use(op_a(inst));
        }

        IrCmd::JUMP_IF_TRUTHY | IrCmd::JUMP_IF_FALSY => {
            visitor.r#use(op_a(inst), 0);
        }

        IrCmd::JUMP_EQ_TAG => {
            visitor.maybe_use(op_a(inst));
        }

        IrCmd::DO_ARITH => {
            visitor.maybe_use(op_b(inst.clone()));
            visitor.maybe_use(op_c(inst.clone()));
            visitor.def(op_a(inst), 0);
        }

        IrCmd::GET_TABLE => {
            visitor.r#use(op_b(inst.clone()), 0);
            visitor.maybe_use(op_c(inst.clone()));
            visitor.def(op_a(inst), 0);
        }

        IrCmd::SET_TABLE => {
            visitor.r#use(op_a(inst), 0);
            visitor.r#use(op_b(inst.clone()), 0);
            visitor.maybe_use(op_c(inst.clone()));
        }

        IrCmd::DO_LEN => {
            visitor.r#use(op_b(inst.clone()), 0);
            visitor.def(op_a(inst), 0);
        }

        IrCmd::GET_CACHED_IMPORT => {
            visitor.def(op_a(inst), 0);
        }

        IrCmd::CONCAT => {
            let start = vm_reg_op(op_a(inst));
            let count = function.uint_op(op_b(inst.clone())) as i32;
            visitor.use_range(start, count);
            visitor.def_range(start, count);
        }

        IrCmd::GET_UPVALUE => {
            // break;
        }

        IrCmd::SET_UPVALUE => {
            // break;
        }

        IrCmd::INTERRUPT => {
            // break;
        }

        IrCmd::BARRIER_OBJ | IrCmd::BARRIER_TABLE_FORWARD => {
            visitor.maybe_use(op_b(inst.clone()));
        }

        IrCmd::CLOSE_UPVALS => {
            // break;
        }

        IrCmd::CAPTURE => {
            visitor.maybe_use(op_a(inst));
            if function.uint_op(op_b(inst.clone())) == 1 {
                visitor.capture(vm_reg_op(op_a(inst)));
            }
        }

        IrCmd::SETLIST => {
            visitor.r#use(op_b(inst.clone()), 0);
            visitor.use_range(
                vm_reg_op(op_c(inst.clone())),
                function.int_op(op_d(inst.clone())),
            );
        }

        IrCmd::CALL => {
            let ra = vm_reg_op(op_a(inst));
            visitor.r#use(op_a(inst), 0);
            visitor.use_range(ra + 1, function.int_op(op_b(inst.clone())));
            visitor.def_range(ra, function.int_op(op_c(inst.clone())));
        }

        IrCmd::RETURN => {
            visitor.use_range(vm_reg_op(op_a(inst)), function.int_op(op_b(inst.clone())));
        }

        IrCmd::FASTCALL => {
            visitor.r#use(op_c(inst.clone()), 0);
            visitor.def_range(
                vm_reg_op(op_b(inst.clone())),
                function.int_op(op_d(inst.clone())),
            );
        }

        IrCmd::INVOKE_FASTCALL => {
            let count = function.int_op(op_f(inst.clone()));
            if count != -1 {
                // Only LOP_FASTCALL3 lowering is allowed to have third optional argument
                if count >= 3 && op_e(inst.clone()).kind() == IrOpKind::Undef {
                    CODEGEN_ASSERT!(
                        op_d(inst.clone()).kind() == IrOpKind::VmReg
                            && vm_reg_op(op_d(inst.clone())) == vm_reg_op(op_c(inst.clone())) + 1
                    );
                    visitor.use_range(vm_reg_op(op_c(inst.clone())), count);
                } else {
                    if count >= 1 {
                        visitor.r#use(op_c(inst.clone()), 0);
                    }
                    if count >= 2 {
                        visitor.maybe_use(op_d(inst.clone()));
                    }
                    if count >= 3 {
                        visitor.maybe_use(op_e(inst.clone()));
                    }
                }
            } else {
                visitor.use_varargs(vm_reg_op(op_c(inst.clone())) as u8);
            }

            visitor.def_range(
                vm_reg_op(op_b(inst.clone())),
                function.int_op(op_g(inst.clone())),
            );
        }

        IrCmd::FORGLOOP => {
            visitor.r#use(op_a(inst), 1);
            visitor.r#use(op_a(inst), 2);
            visitor.def(op_a(inst), 2);
            visitor.def_range(
                vm_reg_op(op_a(inst)) + 3,
                function.int_op(op_b(inst.clone())),
            );
        }

        IrCmd::FORGLOOP_FALLBACK => {
            visitor.use_range(vm_reg_op(op_a(inst)), 3);
            visitor.def(op_a(inst), 2);
            visitor.def_range(
                vm_reg_op(op_a(inst)) + 3,
                (function.int_op(op_b(inst.clone())) as u8) as i32,
            );
        }

        IrCmd::FORGPREP_XNEXT_FALLBACK => {
            visitor.r#use(op_b(inst.clone()), 0);
        }

        IrCmd::FALLBACK_GETGLOBAL => {
            visitor.def(op_b(inst.clone()), 0);
        }

        IrCmd::FALLBACK_SETGLOBAL => {
            visitor.r#use(op_b(inst.clone()), 0);
        }

        IrCmd::FALLBACK_GETTABLEKS => {
            visitor.r#use(op_c(inst.clone()), 0);
            visitor.def(op_b(inst.clone()), 0);
        }

        IrCmd::FALLBACK_SETTABLEKS => {
            visitor.r#use(op_b(inst.clone()), 0);
            visitor.r#use(op_c(inst.clone()), 0);
        }

        IrCmd::FALLBACK_NAMECALL => {
            visitor.r#use(op_c(inst.clone()), 0);
            visitor.def_range(vm_reg_op(op_b(inst.clone())), 2);
        }

        IrCmd::FALLBACK_PREPVARARGS => {
            // break;
        }

        IrCmd::FALLBACK_GETVARARGS => {
            visitor.def_range(
                vm_reg_op(op_b(inst.clone())),
                function.int_op(op_c(inst.clone())),
            );
        }

        IrCmd::FALLBACK_DUPCLOSURE => {
            visitor.def(op_b(inst.clone()), 0);
        }

        IrCmd::FALLBACK_FORGPREP => {
            let start = vm_reg_op(op_b(inst.clone()));
            visitor.use_range(start, 3);
            visitor.def_range(start, 3);
        }

        IrCmd::ADJUST_STACK_TO_REG => {
            visitor.def_range(vm_reg_op(op_a(inst)), -1);
        }

        IrCmd::ADJUST_STACK_TO_TOP => {
            // break;
        }

        IrCmd::GET_TYPEOF => {
            visitor.r#use(op_a(inst), 0);
        }

        IrCmd::FINDUPVAL => {
            visitor.r#use(op_a(inst), 0);
        }

        IrCmd::MARK_USED => {
            visitor.use_range(vm_reg_op(op_a(inst)), function.int_op(op_b(inst.clone())));
        }

        IrCmd::MARK_DEAD => {
            // break;
        }

        _ => {
            // All instructions which reference registers have to be handled explicitly
            // for (auto& op : inst.ops) CODEGEN_ASSERT(op.kind != IrOpKind::VmReg);
            for op in unsafe { (*inst).ops.as_slice() } {
                CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
            }
        }
    }
}
