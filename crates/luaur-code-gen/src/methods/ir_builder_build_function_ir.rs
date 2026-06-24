use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::after_inst_for_n_loop::after_inst_for_n_loop;
use crate::functions::analyze_bytecode_types::analyze_bytecode_types;
use crate::functions::before_inst_for_n_prep::before_inst_for_n_prep;
use crate::functions::build_argument_type_checks::build_argument_type_checks;
use crate::functions::get_op_length::get_op_length;
use crate::functions::has_typed_parameters::has_typed_parameters;
use crate::functions::is_block_terminator::is_block_terminator;
use crate::functions::load_bytecode_type_info::load_bytecode_type_info;
use crate::functions::update_use_counts::update_use_counts;
use crate::records::bytecode_mapping::BytecodeMapping;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;

impl IrBuilder {
    pub fn build_function_ir(&mut self, proto: *mut luaur_vm::records::proto::Proto) {
        unsafe {
            self.function.proto = proto;
            self.function.variadic = (*proto).is_vararg != 0;

            load_bytecode_type_info(&mut self.function);

            let generate_type_checks = has_typed_parameters(&self.function.bc_type_info);
            let mut entry = if generate_type_checks {
                self.block(IrBlockKind::Internal)
            } else {
                IrOp::default()
            };

            self.rebuild_bytecode_basic_blocks(proto);
            analyze_bytecode_types(&mut self.function, &*self.host_hooks);

            self.function.bc_mapping.resize(
                (*proto).sizecode as usize,
                BytecodeMapping {
                    ir_location: !0u32,
                    asm_location: !0u32,
                },
            );

            if generate_type_checks {
                self.begin_block(entry);
                build_argument_type_checks(self, entry);

                let block0 = self.block_at_inst(0);
                self.inst_ir_cmd_ir_op(IrCmd::JUMP, block0);
            } else {
                entry = self.block_at_inst(0);
            }

            self.function.entry_block = entry.index();

            let mut i = 0i32;
            while i < (*proto).sizecode {
                let pc = (*proto).code.add(i as usize);
                let op = LuauOpcode::from(LUAU_INSN_OP(*pc) as u8);
                let mut nexti = i + get_op_length(op);
                debug_assert!(nexti <= (*proto).sizecode);

                self.function.bc_mapping[i as usize] = BytecodeMapping {
                    ir_location: self.function.instructions.len() as u32,
                    asm_location: !0u32,
                };

                if self.inst_index_to_block[i as usize] != !0u32 {
                    let block = self.block_at_inst(i as u32);
                    self.begin_block(block);
                    self.function.block_op(block).startpc = i as u32;
                }

                if op == LuauOpcode::LOP_FORNPREP {
                    before_inst_for_n_prep(self, pc, i);
                }

                if !self.in_terminated_block {
                    if self.interrupt_requested {
                        self.interrupt_requested = false;
                        let pcpos = self.const_uint(i as u32);
                        self.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
                    }

                    self.translate_inst(op, pc, i);

                    if self.cmd_skip_target != -1 {
                        nexti = self.cmd_skip_target;
                        self.cmd_skip_target = -1;
                    }
                }

                if op == LuauOpcode::LOP_FORNLOOP {
                    after_inst_for_n_loop(self, pc);
                }

                i = nexti;
                debug_assert!(i <= (*proto).sizecode);

                if (i as usize) < self.inst_index_to_block.len()
                    && self.inst_index_to_block[i as usize] != !0u32
                {
                    if let Some(last) = self.function.instructions.last() {
                        if !is_block_terminator(last.cmd) {
                            let block = self.block_at_inst(i as u32);
                            self.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
                        }
                    }
                }
            }

            update_use_counts(&mut self.function);
        }
    }
}
