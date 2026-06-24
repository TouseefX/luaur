use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::register_x_64::RegisterX64;

use crate::enums::size_x_64::SizeX64;
use crate::macros::rex_b::REX_B;
use crate::macros::rex_force::REX_FORCE;
use crate::macros::rex_w_bit::REX_W_BIT;

impl AssemblyBuilderX64 {
    pub fn place_rex_register_x_64(&mut self, op: RegisterX64) {
        let code: u8 = REX_W_BIT!(op.size() == SizeX64::qword) | REX_B(op) | REX_FORCE(op);

        if code != 0 {
            self.place(code | 0x40);
        }
    }
}
