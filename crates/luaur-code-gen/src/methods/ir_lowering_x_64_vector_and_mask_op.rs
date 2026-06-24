use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl IrLoweringX64 {
    #[inline]
    pub fn vector_and_mask_op(&mut self) -> OperandX64 {
        if self.vector_and_mask.base.bits == 0xFF {
            self.vector_and_mask =
                AssemblyBuilderX64::u32x4(unsafe { &mut *self.build }, !0u32, !0u32, !0u32, 0u32);
        }

        self.vector_and_mask
    }
}
