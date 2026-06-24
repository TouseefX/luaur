use crate::functions::serialize_lowering_stats::serialize_lowering_stats;
use crate::macros::write_name::WRITE_NAME;
use crate::macros::write_pair::WRITE_PAIR;
use crate::records::compile_stats::CompileStats;
use luaur_code_gen::records::lowering_stats::LoweringStats;

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

pub fn serialize_compile_stats(fp: *mut FILE, stats: &CompileStats) {
    unsafe {
        fprintf(fp, c"{\n".as_ptr());

        WRITE_PAIR!(fp, stats, "        ", lines, "%zu,\n");
        WRITE_PAIR!(fp, stats, "        ", bytecode, "%zu,\n");
        WRITE_PAIR!(fp, stats, "        ", bytecode_instruction_count, "%zu,\n");
        WRITE_PAIR!(fp, stats, "        ", codegen, "%zu,\n");
        WRITE_PAIR!(fp, stats, "        ", read_time, "%f,\n");
        WRITE_PAIR!(fp, stats, "        ", misc_time, "%f,\n");
        WRITE_PAIR!(fp, stats, "        ", parse_time, "%f,\n");
        WRITE_PAIR!(fp, stats, "        ", compile_time, "%f,\n");
        WRITE_PAIR!(fp, stats, "        ", codegen_time, "%f,\n");

        WRITE_NAME!(fp, "        ", lower_stats);
        serialize_lowering_stats(fp as *mut core::ffi::c_void, &stats.lower_stats);

        fprintf(fp, c"\n    }".as_ptr());
    }
}
