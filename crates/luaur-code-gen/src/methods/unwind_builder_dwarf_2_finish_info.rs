use crate::functions::writeu_32::writeu_32;
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

impl UnwindBuilderDwarf2 {
    pub fn finish_info(&mut self) {
        // Terminate section
        self.pos = unsafe { writeu_32(self.pos, 0) };

        luaur_common::LUAU_ASSERT!(self.get_unwind_info_size(0) <= Self::kRawDataLimit as usize);
    }
}
