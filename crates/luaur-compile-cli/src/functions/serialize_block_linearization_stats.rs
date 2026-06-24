use crate::macros::write_pair::WRITE_PAIR;
use luaur_code_gen::records::block_linearization_stats::BlockLinearizationStats;

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

extern "C" {
    pub fn fprintf(fp: *mut FILE, format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

#[allow(non_camel_case_types)]
pub mod libc {
    pub use super::fprintf;
}

#[repr(C)]
struct BlockLinearizationStatsRepr {
    const_prop_instruction_count: u32,
    time_seconds: f64,
}

pub fn serialize_block_linearization_stats(fp: *mut FILE, stats: &BlockLinearizationStats) {
    let stats_repr = unsafe {
        &*(stats as *const BlockLinearizationStats as *const BlockLinearizationStatsRepr)
    };

    unsafe {
        fprintf(fp, c"{\n".as_ptr());

        WRITE_PAIR!(
            fp,
            stats_repr,
            "                ",
            const_prop_instruction_count,
            "%u,\n"
        );
        WRITE_PAIR!(fp, stats_repr, "                ", time_seconds, "%f\n");

        fprintf(fp, c"            }".as_ptr());
    }
}
