use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_b::op_b;
use crate::records::array_value_entry::ArrayValueEntry;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn forward_table_store_to_load(
        &mut self,
        target_addr: &mut IrInst,
        write_offset_op: IrOp,
        inst_idx: u32,
    ) {
        if target_addr.cmd == IrCmd::GET_SLOT_NODE_ADDR {
            CODEGEN_ASSERT!(unsafe { &*self.function }.int_op(write_offset_op) == 0);

            let function = unsafe { &*self.function };
            let key = function.get_inst_index(target_addr);
            *self.hash_value_cache.get_or_insert(key) = inst_idx;
        } else if target_addr.cmd == IrCmd::GET_ARR_ADDR {
            let offset_op = {
                let function = unsafe { &mut *self.function };
                let load_offset = function.as_int_op(write_offset_op).unwrap_or(0);
                let op_b_inst = op_b(target_addr.clone());

                if op_b_inst.kind() == IrOpKind::Constant {
                    let array_addr_offset = function.int_op(op_b_inst)
                        * core::mem::size_of::<luaur_vm::records::lua_t_value::TValue>() as i32;

                    if array_addr_offset != 0 || load_offset == 0 {
                        CODEGEN_ASSERT!(load_offset == 0);
                        unsafe { &mut *self.build }.const_int(array_addr_offset)
                    } else {
                        CODEGEN_ASSERT!(write_offset_op.kind() == IrOpKind::Constant);
                        write_offset_op
                    }
                } else {
                    CODEGEN_ASSERT!(op_b_inst.kind() == IrOpKind::Inst);
                    CODEGEN_ASSERT!(load_offset == 0);
                    op_b_inst
                }
            };

            let function = unsafe { &*self.function };
            let key = function.get_inst_index(target_addr);
            self.array_value_cache.push(ArrayValueEntry {
                pointer: key,
                offset: offset_op,
                value: inst_idx,
            });
        } else {
            CODEGEN_ASSERT!(
                target_addr.cmd == IrCmd::TABLE_SETNUM
                    || target_addr.cmd == IrCmd::GET_CLOSURE_UPVAL_ADDR
            );
        }
    }
}
