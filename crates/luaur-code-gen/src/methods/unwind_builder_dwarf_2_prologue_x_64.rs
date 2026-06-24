use crate::enums::size_x_64::SizeX64;
use crate::functions::advance_location::advance_location;
use crate::functions::define_cfa_expression_offset::define_cfa_expression_offset;
use crate::functions::define_saved_register_location::define_saved_register_location;
use crate::functions::reg_index_to_dw_reg_x_64::reg_index_to_dw_reg_x_64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::dw_reg_x_64_rbp::DW_REG_X64_RBP;
use crate::records::register_x_64::RegisterX64;
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

impl UnwindBuilderDwarf2 {
    pub fn prologue_x_64(
        &mut self,
        prologue_size: u32,
        stack_size: u32,
        setup_frame: bool,
        gpr: &[RegisterX64],
        simd: &[RegisterX64],
    ) {
        unsafe {
            CODEGEN_ASSERT!(stack_size > 0 && stack_size < 4096 && stack_size % 8 == 0);

            let mut stack_offset: u32 = 8; // Return address was pushed by calling the function
            let mut prologue_offset: u32 = 0;

            if setup_frame {
                // push rbp
                stack_offset += 8;
                prologue_offset += 2;
                self.pos = advance_location(self.pos, 2);
                self.pos = define_cfa_expression_offset(self.pos, stack_offset);
                self.pos = define_saved_register_location(self.pos, DW_REG_X64_RBP, stack_offset);

                // mov rbp, rsp
                prologue_offset += 3;
                self.pos = advance_location(self.pos, 3);
            }

            // push reg
            for reg in gpr.iter() {
                CODEGEN_ASSERT!((*reg).size() == SizeX64::qword);

                stack_offset += 8;
                prologue_offset += 2;
                self.pos = advance_location(self.pos, 2);
                self.pos = define_cfa_expression_offset(self.pos, stack_offset);
                self.pos = define_saved_register_location(
                    self.pos,
                    reg_index_to_dw_reg_x_64(reg.index()),
                    stack_offset,
                );
            }

            CODEGEN_ASSERT!(simd.is_empty());

            // sub rsp, stackSize
            stack_offset += stack_size;
            prologue_offset += if stack_size >= 128 { 7 } else { 4 };
            self.pos = advance_location(self.pos, 4);
            self.pos = define_cfa_expression_offset(self.pos, stack_offset);

            CODEGEN_ASSERT!(stack_offset % 16 == 0);
            CODEGEN_ASSERT!(prologue_offset == prologue_size);
        }
    }
}
