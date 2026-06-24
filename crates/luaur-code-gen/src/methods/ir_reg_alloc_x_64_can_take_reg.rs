use crate::enums::size_x_64::SizeX64;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::register_x_64::RegisterX64;

impl IrRegAllocX64 {
    pub fn can_take_reg(&self, reg: RegisterX64) -> bool {
        let free_map = if reg.size() == SizeX64::xmmword {
            &self.free_xmm_map
        } else {
            &self.free_gpr_map
        };

        let inst_users = if reg.size() == SizeX64::xmmword {
            &self.xmm_inst_users
        } else {
            &self.gpr_inst_users
        };

        free_map[reg.index() as usize]
            || inst_users[reg.index() as usize] != crate::records::ir_data::k_invalid_inst_idx
    }
}
