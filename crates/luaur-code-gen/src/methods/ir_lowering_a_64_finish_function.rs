use crate::enums::code_gen_counter::CodeGenCounter;
use crate::enums::kind_a_64::KindA64;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::register_a_64::RegisterA64;
use crate::type_aliases::instruction_ir_builder::Instruction;

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const X0: RegisterA64 = reg(KindA64::x, 0);
const X1: RegisterA64 = reg(KindA64::x, 1);

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_finish_function(&mut self) {
        unsafe {
            if (*self.build).log_text {
                (*self.build).log_append(format_args!("; interrupt handlers\n"));
            }

            for i in 0..self.interrupt_handlers.len() {
                let mut handler = self.interrupt_handlers[i];
                (*self.build).set_label_label(&mut handler.self_);
                (*self.build).mov_register_a_64_i32(
                    X0,
                    ((handler.pcpos + 1) * core::mem::size_of::<Instruction>() as u32) as i32,
                );
                (*self.build).adr_register_a_64_label(X1, &mut handler.next);
                (*self.build).b_label(&mut (*self.helpers).interrupt);
            }

            if (*self.build).log_text {
                (*self.build).log_append(format_args!("; exit handlers\n"));
            }

            for i in 0..self.exit_handlers.len() {
                let mut handler = self.exit_handlers[i];
                if handler.pcpos == K_VM_EXIT_ENTRY_GUARD_PC {
                    (*self.build).set_label_label(&mut handler.self_);

                    self.ir_lowering_a_64_alloc_and_increment_counter_at(
                        CodeGenCounter::VmExitTaken,
                        !0u32,
                    );

                    (*self.build).b_label(&mut (*self.helpers).exitContinueVmClearNativeFlag);
                } else {
                    (*self.build).set_label_label(&mut handler.self_);

                    self.ir_lowering_a_64_alloc_and_increment_counter_at(
                        CodeGenCounter::VmExitTaken,
                        handler.pcpos,
                    );

                    (*self.build).mov_register_a_64_i32(
                        X0,
                        (handler.pcpos * core::mem::size_of::<Instruction>() as u32) as i32,
                    );
                    (*self.build).b_label(&mut (*self.helpers).updatePcAndContinueInVm);
                }
            }

            let end = (*self.build).set_label();
            (*self.function).end_location = (*self.build).get_label_offset(&end);
            (*self.build).udf();

            if !self.stats.is_null() {
                if (*self).error {
                    (*self.stats).lowering_errors += 1;
                }

                if self.regs.error {
                    (*self.stats).reg_alloc_errors += 1;
                }
            }
        }
    }
}
