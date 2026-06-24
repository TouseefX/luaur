//! @interface-stub
use alloc::string::String;
use alloc::vec::Vec;

use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::enums::code_gen_flags::CodeGenFlags;
use crate::functions::assemble_helpers_code_gen_a_64::assemble_helpers as assemble_helpers_a_64;
use crate::functions::assemble_helpers_code_gen_x_64::assemble_helpers as assemble_helpers_x_64;
use crate::functions::gather_functions::gather_functions;
use crate::functions::get_instruction_count_code_gen_assembly::get_instruction_count_instruction_size;
use crate::functions::log_function_header::log_function_header;
use crate::functions::log_function_types::log_function_types;
use crate::functions::lower_function::{lower_function_a_64, lower_function_x_64};
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::function_bytecode_summary::FunctionBytecodeSummary;
use crate::records::function_stats::FunctionStats;
use crate::records::ir_builder::IrBuilder;
use crate::records::lowering_stats::{FunctionStats_Enable, LoweringStats};
use crate::records::module_helpers::ModuleHelpers;
use crate::traits::LogAppend;
use luaur_common::enums::luau_proto_flag::LuauProtoFlag;
use luaur_vm::macros::clvalue::clvalue;
use luaur_vm::macros::getstr::getstr;
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::t_value::TValue;

impl LogAppend for AssemblyBuilderX64 {
    fn log_append(&mut self, args: core::fmt::Arguments<'_>) {
        self.log_append(args);
    }
}

impl LogAppend for AssemblyBuilderA64 {
    fn log_append(&mut self, args: core::fmt::Arguments<'_>) {
        self.log_append(args);
    }
}

pub unsafe fn get_assembly_impl_x_64(
    build: &mut AssemblyBuilderX64,
    func: *const TValue,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    let cl = clvalue!(func);
    let root: *mut Proto = (*(*cl).inner.l).p;

    if (options.compilation_options.flags & CodeGenFlags::CodeGen_OnlyNativeModules as u32) != 0
        && ((*root).flags & LuauProtoFlag::LPF_NATIVE_MODULE as u8) == 0
    {
        build.finalize();
        return String::new();
    }

    let mut protos: Vec<*mut Proto> = Vec::new();
    gather_functions(
        &mut protos,
        root,
        options.compilation_options.flags,
        ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    );

    protos.retain(|p| !p.is_null());

    if !stats.is_null() {
        (*stats).total_functions += protos.len() as u32;
    }

    if protos.is_empty() {
        build.finalize();
        return String::new();
    }

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

    for p in protos {
        let mut ir = IrBuilder::ir_builder_ir_builder(&options.compilation_options.hooks);
        ir.build_function_ir(p);
        let mut asm_size = build.get_code_size();
        let mut asm_count = build.get_instruction_count();

        if options.include_assembly || options.include_ir {
            log_function_header(build, p);
        }

        if options.include_ir_types {
            log_function_types(
                build,
                &ir.function,
                options.compilation_options.userdata_types,
            );
        }

        let mut result = CodeGenCompilationResult::Success;

        if !lower_function_x_64(
            &mut ir,
            build,
            &mut helpers,
            p,
            options.clone(),
            stats,
            &mut result,
        ) {
            if build.log_text {
                build.log_append(format_args!("; skipping (can't lower)\n"));
            }

            asm_size = 0;
            asm_count = 0;

            if !stats.is_null() {
                (*stats).skipped_functions += 1;
            }
        } else {
            asm_size = build.get_code_size().wrapping_sub(asm_size);
            asm_count = build.get_instruction_count().wrapping_sub(asm_count);
        }

        if !stats.is_null() && ((*stats).function_stats_flags & FunctionStats_Enable) != 0 {
            let mut function_stat = FunctionStats::default();

            function_stat.name = if !(*p).debugname.is_null() {
                let name = getstr((*p).debugname as *const _);
                core::ffi::CStr::from_ptr(name)
                    .to_string_lossy()
                    .into_owned()
            } else if (*p).bytecodeid == (*root).bytecodeid {
                String::from("[top level]")
            } else {
                String::from("[anonymous]")
            };
            function_stat.line = (*p).linedefined;
            function_stat.bcode_count =
                get_instruction_count_instruction_size((*p).code, (*p).sizecode as u32);
            function_stat.ir_count = ir.function.instructions.len() as u32;
            function_stat.asm_size = asm_size.wrapping_mul(core::mem::size_of::<u8>() as u32);
            function_stat.asm_count = asm_count;

            if ((*stats).function_stats_flags
                & crate::enums::function_stats_flags::FunctionStatsFlags::FunctionStats_BytecodeSummary
                    as u32)
                != 0
            {
                let summary = FunctionBytecodeSummary::from_proto(p, 0);
                function_stat
                    .bytecode_summary
                    .push(summary.get_counts(0).clone());
            }

            (*stats).functions.push(function_stat);
        }

        if build.log_text {
            build.log_append(format_args!("\n"));
        }
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

pub unsafe fn get_assembly_impl_a_64(
    build: &mut AssemblyBuilderA64,
    func: *const TValue,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    let cl = clvalue!(func);
    let root: *mut Proto = (*(*cl).inner.l).p;

    if (options.compilation_options.flags & CodeGenFlags::CodeGen_OnlyNativeModules as u32) != 0
        && ((*root).flags & LuauProtoFlag::LPF_NATIVE_MODULE as u8) == 0
    {
        build.finalize();
        return String::new();
    }

    let mut protos: Vec<*mut Proto> = Vec::new();
    gather_functions(
        &mut protos,
        root,
        options.compilation_options.flags,
        ((*root).flags & LuauProtoFlag::LPF_NATIVE_FUNCTION as u8) != 0,
    );

    protos.retain(|p| !p.is_null());

    if !stats.is_null() {
        (*stats).total_functions += protos.len() as u32;
    }

    if protos.is_empty() {
        build.finalize();
        return String::new();
    }

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

    for p in protos {
        let mut ir = IrBuilder::ir_builder_ir_builder(&options.compilation_options.hooks);
        ir.build_function_ir(p);
        let mut asm_size = build.get_code_size();
        let mut asm_count = build.get_instruction_count();

        if options.include_assembly || options.include_ir {
            log_function_header(build, p);
        }

        if options.include_ir_types {
            log_function_types(
                build,
                &ir.function,
                options.compilation_options.userdata_types,
            );
        }

        let mut result = CodeGenCompilationResult::Success;

        if !lower_function_a_64(
            &mut ir,
            build,
            &mut helpers,
            p,
            options.clone(),
            stats,
            &mut result,
        ) {
            if build.log_text {
                build.log_append(format_args!("; skipping (can't lower)\n"));
            }

            asm_size = 0;
            asm_count = 0;

            if !stats.is_null() {
                (*stats).skipped_functions += 1;
            }
        } else {
            asm_size = build.get_code_size().wrapping_sub(asm_size);
            asm_count = build.get_instruction_count().wrapping_sub(asm_count);
        }

        if !stats.is_null() && ((*stats).function_stats_flags & FunctionStats_Enable) != 0 {
            let mut function_stat = FunctionStats::default();

            function_stat.name = if !(*p).debugname.is_null() {
                let name = getstr((*p).debugname as *const _);
                core::ffi::CStr::from_ptr(name)
                    .to_string_lossy()
                    .into_owned()
            } else if (*p).bytecodeid == (*root).bytecodeid {
                String::from("[top level]")
            } else {
                String::from("[anonymous]")
            };
            function_stat.line = (*p).linedefined;
            function_stat.bcode_count =
                get_instruction_count_instruction_size((*p).code, (*p).sizecode as u32);
            function_stat.ir_count = ir.function.instructions.len() as u32;
            function_stat.asm_size = asm_size.wrapping_mul(core::mem::size_of::<u32>() as u32);
            function_stat.asm_count = asm_count;

            if ((*stats).function_stats_flags
                & crate::enums::function_stats_flags::FunctionStatsFlags::FunctionStats_BytecodeSummary
                    as u32)
                != 0
            {
                let summary = FunctionBytecodeSummary::from_proto(p, 0);
                function_stat
                    .bytecode_summary
                    .push(summary.get_counts(0).clone());
            }

            (*stats).functions.push(function_stat);
        }

        if build.log_text {
            build.log_append(format_args!("\n"));
        }
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
