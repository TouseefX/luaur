use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_exit_op::vm_exit_op;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_get_target_label<'a>(
        &'a mut self,
        op: IrOp,
        _index: u32,
        fresh: &'a mut Label,
    ) -> &'a mut Label {
        if op.kind() == IrOpKind::Undef {
            return fresh;
        }

        if op.kind() == IrOpKind::VmExit {
            if let Some(index) = self.exit_handler_map.find(&vm_exit_op(op)) {
                return &mut self.exit_handlers[*index as usize].self_;
            }

            return fresh;
        }

        self.ir_lowering_a_64_label_op(op)
    }
}
