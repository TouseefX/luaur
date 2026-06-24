use crate::functions::serialize_block_linearization_stats::{
    serialize_block_linearization_stats, FILE,
};
use crate::functions::serialize_function_stats::serialize_function_stats;
use crate::macros::write_name::WRITE_NAME;
use crate::macros::write_pair::WRITE_PAIR;
use luaur_code_gen::records::block_linearization_stats::BlockLinearizationStats;
use luaur_code_gen::records::function_stats::FunctionStats;
use luaur_code_gen::records::lowering_stats::LoweringStats;

extern "C" {
    pub fn fprintf(fp: *mut FILE, format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}

#[allow(non_camel_case_types)]
pub mod libc {
    pub use super::fprintf;
}

pub fn serialize_lowering_stats(fp: *mut core::ffi::c_void, stats: &LoweringStats) {
    let fp_file = fp as *mut FILE;
    unsafe {
        fprintf(fp_file, c"{\n".as_ptr());

        WRITE_PAIR!(fp_file, stats, "            ", total_functions, "%u,\n");
        WRITE_PAIR!(fp_file, stats, "            ", skipped_functions, "%u,\n");
        WRITE_PAIR!(fp_file, stats, "            ", spills_to_slot, "%d,\n");
        WRITE_PAIR!(fp_file, stats, "            ", spills_to_restore, "%d,\n");
        WRITE_PAIR!(
            fp_file,
            stats,
            "            ",
            max_spill_slots_used,
            "%u,\n"
        );
        WRITE_PAIR!(fp_file, stats, "            ", blocks_pre_opt, "%u,\n");
        WRITE_PAIR!(fp_file, stats, "            ", blocks_post_opt, "%u,\n");
        WRITE_PAIR!(
            fp_file,
            stats,
            "            ",
            max_block_instructions,
            "%u,\n"
        );
        WRITE_PAIR!(fp_file, stats, "            ", reg_alloc_errors, "%d,\n");
        WRITE_PAIR!(fp_file, stats, "            ", lowering_errors, "%d,\n");

        WRITE_NAME!(fp_file, "            ", block_linearization_stats);
        serialize_block_linearization_stats(fp_file, &stats.block_linearization_stats);
        fprintf(fp_file, c",\n".as_ptr());

        WRITE_NAME!(fp_file, "            ", functions);
        let function_count = stats.functions.len();

        if function_count == 0 {
            fprintf(fp_file, c"[]".as_ptr());
        } else {
            fprintf(fp_file, c"[\n".as_ptr());
            for i in 0..function_count {
                serialize_function_stats(fp, &stats.functions[i]);
                if i < function_count - 1 {
                    fprintf(fp_file, c",\n".as_ptr());
                }
            }
            fprintf(fp_file, c"\n            ]".as_ptr());
        }

        fprintf(fp_file, c"\n        }".as_ptr());
    }
}
