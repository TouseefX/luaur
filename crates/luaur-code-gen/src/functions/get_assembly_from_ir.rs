//! @interface-stub
use alloc::string::String;
use alloc::vec::Vec;

use crate::enums::abix_64::ABIX64;
use crate::enums::features_a_64::FeaturesA64;
use crate::enums::target::Target;
use crate::functions::get_assembly_from_ir_impl::{
    get_assembly_from_ir_impl_a_64, get_assembly_from_ir_impl_x_64,
};
use crate::functions::get_cpu_features_a_64::get_cpu_features_a_64;
use crate::functions::get_cpu_features_x_64::get_cpu_features_x_64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::ir_builder::IrBuilder;
use crate::records::lowering_stats::LoweringStats;

pub unsafe fn get_assembly_from_ir(
    ir: &mut IrBuilder,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    match options.target {
        Target::Host => {
            #[cfg(target_arch = "aarch64")]
            {
                let cpu_features = get_cpu_features_a_64();
                let mut build = AssemblyBuilderA64 {
                    data: Vec::new(),
                    code: Vec::new(),
                    text: String::new(),
                    log_text: false,
                    features: 0,
                    next_label: 1,
                    pending_labels: Vec::new(),
                    label_locations: Vec::new(),
                    finalized: false,
                    overflowed: false,
                    data_pos: 0,
                    code_pos: core::ptr::null_mut(),
                    code_end: core::ptr::null_mut(),
                };
                build.assembly_builder_a_64_assembly_builder_a_64(
                    options.include_assembly,
                    cpu_features,
                );

                get_assembly_from_ir_impl_a_64(&mut build, ir, options, stats)
            }

            #[cfg(not(target_arch = "aarch64"))]
            {
                let cpu_features = get_cpu_features_x_64();
                let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(
                    options.include_assembly,
                    cpu_features,
                );

                get_assembly_from_ir_impl_x_64(&mut build, ir, options, stats)
            }
        }

        Target::A64 => {
            let mut build = AssemblyBuilderA64 {
                data: Vec::new(),
                code: Vec::new(),
                text: String::new(),
                log_text: false,
                features: 0,
                next_label: 1,
                pending_labels: Vec::new(),
                label_locations: Vec::new(),
                finalized: false,
                overflowed: false,
                data_pos: 0,
                code_pos: core::ptr::null_mut(),
                code_end: core::ptr::null_mut(),
            };
            build.assembly_builder_a_64_assembly_builder_a_64(
                options.include_assembly,
                FeaturesA64::Feature_JSCVT as u32,
            );

            get_assembly_from_ir_impl_a_64(&mut build, ir, options, stats)
        }

        Target::A64_NoFeatures => {
            let mut build = AssemblyBuilderA64 {
                data: Vec::new(),
                code: Vec::new(),
                text: String::new(),
                log_text: false,
                features: 0,
                next_label: 1,
                pending_labels: Vec::new(),
                label_locations: Vec::new(),
                finalized: false,
                overflowed: false,
                data_pos: 0,
                code_pos: core::ptr::null_mut(),
                code_end: core::ptr::null_mut(),
            };
            build.assembly_builder_a_64_assembly_builder_a_64(options.include_assembly, 0);

            get_assembly_from_ir_impl_a_64(&mut build, ir, options, stats)
        }

        Target::X64_Windows => {
            let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
                options.include_assembly,
                ABIX64::Windows,
                0,
            );

            get_assembly_from_ir_impl_x_64(&mut build, ir, options, stats)
        }

        Target::X64_SystemV => {
            let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_abix_64_i32(
                options.include_assembly,
                ABIX64::SystemV,
                0,
            );

            get_assembly_from_ir_impl_x_64(&mut build, ir, options, stats)
        }
    }
}
