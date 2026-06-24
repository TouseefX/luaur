use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::macros::op_b::op_b;
use crate::macros::op_c::op_c;
use crate::macros::op_d::op_d;
use crate::macros::op_g::op_g;
use crate::records::ir_inst::IrInst;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;

impl IrValueLocationTracking {
    pub fn before_inst_lowering(&mut self, inst: &mut IrInst) {
        match inst.cmd {
            IrCmd::STORE_TAG => self.invalidate_restore_op(op_a(inst), true),
            IrCmd::STORE_EXTRA
            | IrCmd::STORE_POINTER
            | IrCmd::STORE_DOUBLE
            | IrCmd::STORE_INT
            | IrCmd::STORE_INT64
            | IrCmd::STORE_VECTOR
            | IrCmd::STORE_TVALUE
            | IrCmd::STORE_SPLIT_TVALUE => self.invalidate_restore_op(op_a(inst), false),

            IrCmd::ADJUST_STACK_TO_REG => {
                self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1)
            }
            IrCmd::FASTCALL => {
                let function = unsafe { &*self.function };
                self.invalidate_restore_vm_regs(
                    vm_reg_op(op_b(inst.clone())),
                    function.int_op(op_d(inst.clone())),
                );
            }
            IrCmd::INVOKE_FASTCALL => {
                let function = unsafe { &*self.function };
                self.invalidate_restore_vm_regs(
                    vm_reg_op(op_b(inst.clone())),
                    function.int_op(op_g(inst.clone())),
                );
            }

            IrCmd::DO_ARITH | IrCmd::DO_LEN | IrCmd::GET_TABLE | IrCmd::GET_CACHED_IMPORT => {
                self.invalidate_restore_op(op_a(inst), false)
            }
            IrCmd::CONCAT => {
                let function = unsafe { &*self.function };
                self.invalidate_restore_vm_regs(
                    vm_reg_op(op_a(inst)),
                    function.uint_op(op_b(inst.clone())) as i32,
                );
            }
            IrCmd::GET_UPVALUE => {}
            IrCmd::CALL => self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)), -1),
            IrCmd::FORGLOOP | IrCmd::FORGLOOP_FALLBACK => {
                self.invalidate_restore_vm_regs(vm_reg_op(op_a(inst)) + 2, -1)
            }
            IrCmd::FALLBACK_GETGLOBAL | IrCmd::FALLBACK_GETTABLEKS => {
                self.invalidate_restore_op(op_b(inst.clone()), false)
            }
            IrCmd::FALLBACK_NAMECALL => {
                self.invalidate_restore_vm_regs(vm_reg_op(op_b(inst.clone())), 2)
            }
            IrCmd::FALLBACK_GETVARARGS => {
                let function = unsafe { &*self.function };
                self.invalidate_restore_vm_regs(
                    vm_reg_op(op_b(inst.clone())),
                    function.int_op(op_c(inst.clone())),
                );
            }
            IrCmd::FALLBACK_DUPCLOSURE => self.invalidate_restore_op(op_b(inst.clone()), false),
            IrCmd::FALLBACK_FORGPREP => {
                self.invalidate_restore_vm_regs(vm_reg_op(op_b(inst.clone())), 3)
            }

            IrCmd::LOAD_TAG
            | IrCmd::LOAD_POINTER
            | IrCmd::LOAD_DOUBLE
            | IrCmd::LOAD_INT64
            | IrCmd::LOAD_INT
            | IrCmd::LOAD_FLOAT
            | IrCmd::LOAD_TVALUE
            | IrCmd::CMP_ANY
            | IrCmd::CMP_TAG
            | IrCmd::JUMP_IF_TRUTHY
            | IrCmd::JUMP_IF_FALSY
            | IrCmd::JUMP_EQ_TAG
            | IrCmd::SELECT_INT64
            | IrCmd::SET_TABLE
            | IrCmd::SET_UPVALUE
            | IrCmd::INTERRUPT
            | IrCmd::BARRIER_OBJ
            | IrCmd::BARRIER_TABLE_FORWARD
            | IrCmd::CLOSE_UPVALS
            | IrCmd::CAPTURE
            | IrCmd::SETLIST
            | IrCmd::RETURN
            | IrCmd::FORGPREP_XNEXT_FALLBACK
            | IrCmd::FALLBACK_SETGLOBAL
            | IrCmd::FALLBACK_SETTABLEKS
            | IrCmd::FALLBACK_PREPVARARGS
            | IrCmd::ADJUST_STACK_TO_TOP
            | IrCmd::GET_TYPEOF
            | IrCmd::NEWCLOSURE
            | IrCmd::FINDUPVAL
            | IrCmd::CHECK_TAG
            | IrCmd::CHECK_TRUTHY
            | IrCmd::ADD_NUM
            | IrCmd::SUB_NUM
            | IrCmd::MUL_NUM
            | IrCmd::DIV_NUM
            | IrCmd::IDIV_NUM
            | IrCmd::MOD_NUM
            | IrCmd::MIN_NUM
            | IrCmd::MAX_NUM
            | IrCmd::JUMP_CMP_NUM
            | IrCmd::FLOOR_NUM
            | IrCmd::CEIL_NUM
            | IrCmd::ROUND_NUM
            | IrCmd::SQRT_NUM
            | IrCmd::ABS_NUM => {}

            _ => {
                for op in inst.ops.as_slice() {
                    CODEGEN_ASSERT!(op.kind() != IrOpKind::VmReg);
                }
            }
        }
    }
}
