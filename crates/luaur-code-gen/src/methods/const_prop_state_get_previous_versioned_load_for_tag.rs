use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::is_gco::is_gco;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;
use luaur_vm::enums::lua_type::lua_Type;

impl ConstPropState {
    pub fn get_previous_versioned_load_for_tag(&mut self, tag: u8, vm_reg: IrOp) -> (IrCmd, u32) {
        if !self.function.is_null() {
            let cfg = unsafe { &(*self.function).cfg };
            let reg_index = vm_reg_op(vm_reg) as usize;
            let reg_bit = reg_index % 64;
            let reg_array_index = reg_index / 64;
            if (cfg.captured.regs[reg_array_index] & (1u64 << reg_bit)) == 0 {
                if tag == lua_Type::LUA_TBOOLEAN as u8 {
                    if let Some(prev_idx) =
                        self.get_previous_versioned_load_index(IrCmd::LOAD_INT, vm_reg)
                    {
                        return (IrCmd::LOAD_INT, unsafe { *prev_idx });
                    }
                } else if tag == lua_Type::LUA_TNUMBER as u8 {
                    if let Some(prev_idx) =
                        self.get_previous_versioned_load_index(IrCmd::LOAD_DOUBLE, vm_reg)
                    {
                        return (IrCmd::LOAD_DOUBLE, unsafe { *prev_idx });
                    }
                } else if tag == lua_Type::LUA_TINTEGER as u8 {
                    if let Some(prev_idx) =
                        self.get_previous_versioned_load_index(IrCmd::LOAD_INT64, vm_reg)
                    {
                        return (IrCmd::LOAD_INT64, unsafe { *prev_idx });
                    }
                } else if tag == lua_Type::LUA_TVECTOR as u8 {
                    if let Some(prev_idx) =
                        self.get_previous_versioned_load_index(IrCmd::LOAD_FLOAT, vm_reg)
                    {
                        return (IrCmd::LOAD_FLOAT, unsafe { *prev_idx });
                    }
                } else if is_gco(tag) {
                    if let Some(prev_idx) =
                        self.get_previous_versioned_load_index(IrCmd::LOAD_POINTER, vm_reg)
                    {
                        return (IrCmd::LOAD_POINTER, unsafe { *prev_idx });
                    }
                }
            }
        }

        (IrCmd::NOP, !0u32)
    }
}
