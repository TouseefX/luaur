use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::ir_value_kind::IrValueKind;
use crate::enums::size_x_64::SizeX64;
use crate::functions::get_cmd_value_kind::get_cmd_value_kind;
use crate::functions::luau_reg::luau_reg;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::functions::luau_reg_value_int::luau_reg_value_int;
use crate::functions::vm_reg_op::vm_reg_op;
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

fn spill_mem(size: SizeX64, slot: u32) -> OperandX64 {
    mem(size, RegisterX64::rsp, S_SPILL_AREA + slot as i32 * 4)
}

impl IrRegAllocX64 {
    pub fn preserve(&mut self, inst: &mut IrInst) {
        let mut spill = IrSpillX64 {
            inst_idx: unsafe { (&*self.function).get_inst_index(inst) },
            value_kind: get_cmd_value_kind(inst.cmd),
            spill_id: self.next_spill_id,
            stack_slot: IrSpillX64::kNoStackSlot,
            original_loc: inst.reg_x64,
        };
        self.next_spill_id += 1;

        if !unsafe { (&*self.function).has_restore_location_ir_inst_bool(inst, true) } {
            let i = self.find_spill_stack_slot(spill.value_kind);

            if self.is_extra_spill_slot(i) {
                let extra_offset = self.get_extra_spill_address_offset(i);
                let emergency_temp =
                    if inst.reg_x64.size() == SizeX64::xmmword || inst.reg_x64.index() != 11 {
                        RegisterX64::r11
                    } else {
                        RegisterX64::r10
                    };

                let build = unsafe { &mut *self.build };
                build.mov(temp_qword(), OperandX64::reg(emergency_temp));
                build.mov(
                    OperandX64::reg(emergency_temp),
                    mem(
                        SizeX64::qword,
                        r_state(),
                        core::mem::offset_of!(luaur_vm::records::lua_state::lua_State, global)
                            as i32,
                    ),
                );
                build.lea_operand_x_64_operand_x_64(
                    OperandX64::reg(emergency_temp),
                    mem(
                        SizeX64::none,
                        emergency_temp,
                        core::mem::offset_of!(luaur_vm::records::global_state::global_State, ecbdata)
                            as i32
                            + extra_offset,
                    ),
                );

                match spill.value_kind {
                    IrValueKind::Tvalue => build.vmovups(
                        mem(SizeX64::xmmword, emergency_temp, 0),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Double => build.vmovsd_operand_x_64_operand_x_64(
                        mem(SizeX64::qword, emergency_temp, 0),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Pointer | IrValueKind::Int64 => build.mov(
                        mem(SizeX64::qword, emergency_temp, 0),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Tag | IrValueKind::Int => build.mov(
                        mem(SizeX64::dword, emergency_temp, 0),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Float => build.vmovss_operand_x_64_operand_x_64(
                        mem(SizeX64::dword, emergency_temp, 0),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    _ => CODEGEN_ASSERT!(false, "Unsupported value kind"),
                }

                build.mov(OperandX64::reg(emergency_temp), temp_qword());
            } else {
                let build = unsafe { &mut *self.build };
                match spill.value_kind {
                    IrValueKind::Tvalue => build.vmovups(
                        spill_mem(SizeX64::xmmword, i),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Double => build.vmovsd_operand_x_64_operand_x_64(
                        spill_mem(SizeX64::qword, i),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Pointer | IrValueKind::Int64 => {
                        build.mov(spill_mem(SizeX64::qword, i), OperandX64::reg(inst.reg_x64))
                    }
                    IrValueKind::Tag | IrValueKind::Int => {
                        build.mov(spill_mem(SizeX64::dword, i), OperandX64::reg(inst.reg_x64))
                    }
                    IrValueKind::Float => build.vmovss_operand_x_64_operand_x_64(
                        spill_mem(SizeX64::dword, i),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    _ => CODEGEN_ASSERT!(false, "Unsupported value kind"),
                }
            }

            let end =
                i + crate::enums::ir_value_kind::K_VALUE_DWORD_SIZE[spill.value_kind as usize];

            for pos in i..end {
                self.used_spill_slot_halfs[(pos / 64) as usize] |= 1u64 << (pos % 64);
            }

            if (end + 1) / 2 > self.max_used_slot {
                self.max_used_slot = (end + 1) / 2;
            }

            spill.stack_slot = i as u8;
            inst.spilled = true;

            if !self.stats.is_null() {
                unsafe { (*self.stats).spills_to_slot += 1 };
            }
        } else {
            let loc = unsafe { (&*self.function).find_restore_location_ir_inst_bool(inst, true) };

            if loc.lazy {
                CODEGEN_ASSERT!(loc.op.kind() == IrOpKind::VmReg);
                CODEGEN_ASSERT!(loc.conversion_cmd == IrCmd::NOP);

                let store_reg = vm_reg_op(loc.op);
                let build = unsafe { &mut *self.build };
                match spill.value_kind {
                    IrValueKind::Tvalue => {
                        build.vmovups(luau_reg(store_reg), OperandX64::reg(inst.reg_x64))
                    }
                    IrValueKind::Double => build.vmovsd_operand_x_64_operand_x_64(
                        luau_reg_value(store_reg),
                        OperandX64::reg(inst.reg_x64),
                    ),
                    IrValueKind::Pointer | IrValueKind::Int64 => {
                        build.mov(luau_reg_value(store_reg), OperandX64::reg(inst.reg_x64))
                    }
                    IrValueKind::Tag | IrValueKind::Int => {
                        build.mov(luau_reg_value_int(store_reg), OperandX64::reg(inst.reg_x64))
                    }
                    _ => CODEGEN_ASSERT!(false, "Unsupported value kind for lazy store"),
                }

                if spill.value_kind != IrValueKind::Tvalue {
                    build.mov(luau_reg_tag(store_reg), OperandX64::imm(0));
                }

                unsafe { (&mut *self.function).materialize_restore_location(spill.inst_idx) };
            }

            inst.needs_reload = true;

            if !self.stats.is_null() {
                unsafe { (*self.stats).spills_to_restore += 1 };
            }
        }

        self.spills.push(spill);

        self.free_reg(inst.reg_x64);
        inst.reg_x64 = RegisterX64::noreg;
    }
}
