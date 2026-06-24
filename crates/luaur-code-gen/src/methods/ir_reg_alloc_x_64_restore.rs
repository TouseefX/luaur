use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_value_kind::IrValueKind;
use crate::enums::size_x_64::SizeX64;
use crate::functions::qword_reg::qword_reg;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_inst::IrInst;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::ir_spill_x_64::IrSpillX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

const S_TEMPORARY_SLOT: i32 = 64;
const S_SPILL_AREA: i32 = 72;

fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

fn temp_qword() -> OperandX64 {
    mem(SizeX64::qword, RegisterX64::rsp, S_TEMPORARY_SLOT)
}

impl IrRegAllocX64 {
    pub fn restore(&mut self, inst: &mut IrInst, into_original_location: bool) {
        let inst_idx = unsafe { (&*self.function).get_inst_index(inst) };

        let mut i = 0;
        while i < self.spills.len() {
            if self.spills[i].inst_idx == inst_idx {
                let original_loc = self.spills[i].original_loc;
                let reg = if into_original_location {
                    self.take_reg(original_loc, inst_idx)
                } else {
                    self.alloc_reg(original_loc.size(), inst_idx)
                };

                let restore_location =
                    unsafe { (&*self.function).find_restore_location_ir_inst_bool(inst, false) };
                let emergency_temp = if reg.size() == SizeX64::xmmword {
                    RegisterX64::r11
                } else {
                    qword_reg(reg)
                };
                let spill = self.spills[i].clone();

                let mut restore_addr;

                if spill.stack_slot != IrSpillX64::kNoStackSlot {
                    if self.is_extra_spill_slot(spill.stack_slot as u32) {
                        let extra_offset =
                            self.get_extra_spill_address_offset(spill.stack_slot as u32);

                        let build = unsafe { &mut *self.build };
                        if reg.size() == SizeX64::xmmword {
                            build.mov(temp_qword(), OperandX64::reg(emergency_temp));
                        }

                        build.mov(
                            OperandX64::reg(emergency_temp),
                            mem(
                                SizeX64::qword,
                                r_state(),
                                core::mem::offset_of!(
                                    luaur_vm::records::lua_state::lua_State,
                                    global
                                ) as i32,
                            ),
                        );
                        build.lea_operand_x_64_operand_x_64(
                            OperandX64::reg(emergency_temp),
                            mem(
                                SizeX64::none,
                                emergency_temp,
                                core::mem::offset_of!(
                                    luaur_vm::records::global_state::global_State,
                                    ecbdata
                                ) as i32
                                    + extra_offset,
                            ),
                        );

                        restore_addr = mem(reg.size(), emergency_temp, 0);
                    } else {
                        restore_addr = mem(
                            SizeX64::none,
                            RegisterX64::rsp,
                            S_SPILL_AREA + spill.stack_slot as i32 * 4,
                        );
                        restore_addr.memSize = reg.size();
                    }

                    if spill.value_kind == IrValueKind::Double
                        || spill.value_kind == IrValueKind::Int64
                    {
                        restore_addr.memSize = SizeX64::qword;
                    } else if spill.value_kind == IrValueKind::Float {
                        restore_addr.memSize = SizeX64::dword;
                    }

                    let end = spill.stack_slot as u32
                        + crate::enums::ir_value_kind::K_VALUE_DWORD_SIZE
                            [spill.value_kind as usize];

                    for pos in spill.stack_slot as u32..end {
                        self.used_spill_slot_halfs[(pos / 64) as usize] &= !(1u64 << (pos % 64));
                    }
                } else {
                    restore_addr = self.get_restore_address(inst, restore_location);
                }

                let build = unsafe { &mut *self.build };
                match spill.value_kind {
                    IrValueKind::Tvalue => build.vmovups(OperandX64::reg(reg), restore_addr),
                    IrValueKind::Double => {
                        build.vmovsd_operand_x_64_operand_x_64(OperandX64::reg(reg), restore_addr)
                    }
                    IrValueKind::Int if restore_location.kind == IrValueKind::Double => {
                        if restore_location.conversion_cmd == IrCmd::INT_TO_NUM {
                            build.vcvttsd2si(OperandX64::reg(reg), restore_addr);
                        } else if restore_location.conversion_cmd == IrCmd::UINT_TO_NUM {
                            build.vcvttsd2si(OperandX64::reg(qword_reg(reg)), restore_addr);
                        } else {
                            CODEGEN_ASSERT!(
                                false,
                                "re-materialization not supported for this conversion command"
                            );
                        }
                    }
                    IrValueKind::Tag
                    | IrValueKind::Int
                    | IrValueKind::Int64
                    | IrValueKind::Pointer => {
                        build.mov(OperandX64::reg(reg), restore_addr);
                    }
                    IrValueKind::Float => {
                        build.vmovss_operand_x_64_operand_x_64(OperandX64::reg(reg), restore_addr)
                    }
                    _ => CODEGEN_ASSERT!(false, "value kind not supported for restore"),
                }

                if spill.stack_slot != IrSpillX64::kNoStackSlot
                    && self.is_extra_spill_slot(spill.stack_slot as u32)
                {
                    if reg.size() == SizeX64::xmmword {
                        let build = unsafe { &mut *self.build };
                        build.mov(OperandX64::reg(emergency_temp), temp_qword());
                    }
                }

                inst.reg_x64 = reg;
                inst.spilled = false;
                inst.needs_reload = false;

                self.spills[i] = self.spills[self.spills.len() - 1].clone();
                self.spills.pop();
                return;
            }
            i += 1;
        }
    }
}
