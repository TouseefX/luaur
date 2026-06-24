use crate::enums::kind_a_64::KindA64;
use crate::functions::advance_location::advance_location;
use crate::functions::define_cfa_expression_offset::define_cfa_expression_offset;
use crate::functions::define_saved_register_location::define_saved_register_location;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::register_a_64::RegisterA64;
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

impl UnwindBuilderDwarf2 {
    pub fn prologue_a_64(&mut self, prologue_size: u32, stack_size: u32, regs: &[RegisterA64]) {
        unsafe {
            CODEGEN_ASSERT!(stack_size % 16 == 0);
            CODEGEN_ASSERT!(
                regs.len() >= 2
                    && (*regs.get_unchecked(0)).index() == 29
                    && (*regs.get_unchecked(1)).index() == 30
            );
            CODEGEN_ASSERT!((regs.len() as u32) * 8 <= stack_size);

            self.pos = advance_location(self.pos, 4);
            self.pos = define_cfa_expression_offset(self.pos, stack_size);

            self.pos = advance_location(self.pos, prologue_size - 4);

            for i in 0..regs.len() {
                let reg = unsafe { regs.get_unchecked(i) };
                CODEGEN_ASSERT!((*reg).kind() == KindA64::x);
                self.pos = define_saved_register_location(
                    self.pos,
                    (*reg).index() as i32,
                    stack_size - (i as u32 * 8),
                );
            }
        }
    }
}
