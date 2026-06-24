use crate::enums::kind::Kind;
use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::patch::Patch;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn place_btr(
        &mut self,
        name: *const core::ffi::c_char,
        label: &mut Label,
        op: u8,
        cond: RegisterA64,
        bit: u8,
    ) {
        debug_assert!(
            cond.kind() == KindA64::x || cond.kind() == KindA64::w,
            "cond.kind() == KindA64::x || cond.kind() == KindA64::w"
        );
        debug_assert!(
            bit < (if cond.kind() == KindA64::x { 64 } else { 32 }),
            "bit < (cond.kind() == KindA64::x ? 64 : 32)"
        );

        self.place(
            cond.index() as u32
                | (((bit & 0x1f) as u32) << 19)
                | ((op as u32) << 24)
                | (((bit >> 5) as u32) << 31),
        );
        self.commit();

        self.patch_label(label, Kind::Imm14);

        if self.log_text {
            self.log_c_char_register_a_64_label_i32(name, cond, *label, bit as i32);
        }
    }
}
