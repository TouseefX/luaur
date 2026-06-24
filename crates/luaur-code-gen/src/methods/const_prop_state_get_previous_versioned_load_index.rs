use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::has_side_effects::has_side_effects;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn get_previous_versioned_load_index(
        &mut self,
        cmd: IrCmd,
        vm_reg: IrOp,
    ) -> Option<*mut u32> {
        CODEGEN_ASSERT!(vm_reg.kind() == IrOpKind::VmReg);

        let reg = vm_reg_op(vm_reg) as usize;
        let mut versioned_reg = vm_reg;
        versioned_reg = IrOp::ir_op_ir_op_kind_u32(
            IrOpKind::VmReg,
            (vm_reg_op(versioned_reg) as u32) | (self.regs[reg].version << 8),
        );
        let mut ops = crate::type_aliases::ir_ops::IrOps::new();
        ops.push(versioned_reg);
        let versioned_load = IrInst {
            cmd,
            ops,
            last_use: 0,
            use_count: 0,
            reg_x64: Default::default(),
            reg_a64: Default::default(),
            reused_reg: false,
            spilled: false,
            needs_reload: false,
        };

        let prev_idx = match self.value_map.find_mut(&versioned_load) {
            Some(prev_idx) => prev_idx as *mut u32,
            None => return None,
        };

        let inst = unsafe {
            let instructions = &(*self.function).instructions;
            &instructions[*prev_idx as usize]
        };
        if inst.use_count != 0 || has_side_effects(inst.cmd) {
            Some(prev_idx)
        } else {
            None
        }
    }
}
