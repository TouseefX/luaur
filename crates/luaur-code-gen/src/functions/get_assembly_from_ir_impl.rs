//! @interface-stub
use alloc::string::String;
use alloc::vec::Vec;

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::functions::assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64;
use crate::functions::assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64;
use crate::functions::lower_function::{lower_function_a_64, lower_function_x_64};
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::ir_builder::IrBuilder;
use crate::records::lowering_stats::LoweringStats;
use crate::records::module_helpers::ModuleHelpers;

pub unsafe fn get_assembly_from_ir_impl_x_64(
    build: &mut AssemblyBuilderX64,
    ir: &mut IrBuilder,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    let mut helpers = ModuleHelpers::default();
    assemble_helpers_x_64(build, &mut helpers);

    if !options.include_outlined_code && options.include_assembly {
        build.text.clear();
        build.log_append(format_args!(
            "; skipping {} bytes of outlined helpers\n",
            build
                .get_code_size()
                .wrapping_mul(core::mem::size_of::<u8>() as u32)
        ));
    }

    let mut result = CodeGenCompilationResult::Success;

    if !lower_function_x_64(
        ir,
        build,
        &mut helpers,
        core::ptr::null_mut(),
        options.clone(),
        stats,
        &mut result,
    ) {
        if build.log_text {
            build.log_append(format_args!("; skipping (can't lower)\n"));
        }
    }

    if build.log_text {
        build.log_append(format_args!("\n"));
    }

    if !build.finalize() {
        return String::new();
    }

    if options.output_binary {
        let mut bytes = Vec::with_capacity(build.code.len() + build.data.len());
        bytes.extend_from_slice(&build.code);
        bytes.extend_from_slice(&build.data);
        String::from_utf8_unchecked(bytes)
    } else {
        build.text.clone()
    }
}

pub unsafe fn get_assembly_from_ir_impl_a_64(
    build: &mut AssemblyBuilderA64,
    ir: &mut IrBuilder,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    let mut helpers = ModuleHelpers::default();
    assemble_helpers_a_64(build, &mut helpers);

    if !options.include_outlined_code && options.include_assembly {
        build.text.clear();
        build.log_append(format_args!(
            "; skipping {} bytes of outlined helpers\n",
            build
                .get_code_size()
                .wrapping_mul(core::mem::size_of::<u32>() as u32)
        ));
    }

    let mut result = CodeGenCompilationResult::Success;

    if !lower_function_a_64(
        ir,
        build,
        &mut helpers,
        core::ptr::null_mut(),
        options.clone(),
        stats,
        &mut result,
    ) {
        if build.log_text {
            build.log_append(format_args!("; skipping (can't lower)\n"));
        }
    }

    if build.log_text {
        build.log_append(format_args!("\n"));
    }

    if !build.finalize() {
        return String::new();
    }

    if options.output_binary {
        let code = core::slice::from_raw_parts(
            build.code.as_ptr().cast::<u8>(),
            build.code.len() * core::mem::size_of::<u32>(),
        );
        let mut bytes = Vec::with_capacity(code.len() + build.data.len());
        bytes.extend_from_slice(code);
        bytes.extend_from_slice(&build.data);
        String::from_utf8_unchecked(bytes)
    } else {
        build.text.clone()
    }
}
