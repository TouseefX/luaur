use alloc::string::String;
use core::ffi::{c_char, c_void, CStr};
use std::io::Write;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_cli_lib::functions::read_file::read_file;
use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
use luaur_code_gen::enums::include_cfg_info::IncludeCfgInfo;
use luaur_code_gen::enums::include_ir_prefix::IncludeIrPrefix;
use luaur_code_gen::enums::include_reg_flow_info::IncludeRegFlowInfo;
use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
use luaur_code_gen::enums::target::Target;
use luaur_code_gen::records::assembly_options::AssemblyOptions;
use luaur_code_gen::records::compilation_options::CompilationOptions;
use luaur_common::functions::get_clock::get_clock;
use luaur_compiler::functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options;
use luaur_compiler::records::compile_error::CompileError;

use crate::enums::compile_format::CompileFormat;
use crate::functions::annotate_instruction::annotate_instruction;
use crate::functions::copts::copts;
use crate::functions::get_codegen_assembly::get_codegen_assembly;
use crate::functions::record_delta_time::record_delta_time;
use crate::functions::report_error_compile::report_error_c_char_luau_parse_error;
use crate::functions::report_error_compile_alt_b::report_error_c_char_luau_compile_error;
use crate::records::compile_stats::CompileStats;
use crate::records::global_options::globalOptions;

pub fn compile_file(
    name: *const c_char,
    format: CompileFormat,
    assembly_target: Target,
    stats: &mut CompileStats,
    dump_constants: bool,
) -> bool {
    let mut currts = get_clock();
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = name_cstr.to_string_lossy();
    let name_with_nul = String::from_utf8_lossy(name_cstr.to_bytes_with_nul()).into_owned();

    let Some(source) = read_file(&name_with_nul) else {
        eprintln!("Error opening {}", name_str);
        return false;
    };

    stats.read_time += record_delta_time(&mut currts);

    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut bcb = BytecodeBuilder::new(None);

        let mut options = AssemblyOptions {
            target: assembly_target,
            compilation_options: CompilationOptions::default(),
            output_binary: format == CompileFormat::CodegenNull,
            include_assembly: false,
            include_ir: false,
            include_outlined_code: false,
            include_ir_types: false,
            include_ir_prefix: IncludeIrPrefix::default(),
            include_use_info: IncludeUseInfo::default(),
            include_cfg_info: IncludeCfgInfo::default(),
            include_reg_flow_info: IncludeRegFlowInfo::default(),
            annotator: Some(annotate_instruction),
            annotator_context: &mut bcb as *mut BytecodeBuilder as *mut c_void,
        };
        options.compilation_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;

        if !options.output_binary {
            options.include_assembly = format != CompileFormat::CodegenIr;
            options.include_ir = format != CompileFormat::CodegenAsm;
            options.include_ir_types = format != CompileFormat::CodegenAsm;
            options.include_outlined_code = format == CompileFormat::CodegenVerbose;
        }

        if format == CompileFormat::Text {
            let mut flags = BytecodeBuilder::DUMP_CODE
                | BytecodeBuilder::DUMP_SOURCE
                | BytecodeBuilder::DUMP_LOCALS
                | BytecodeBuilder::DUMP_REMARKS
                | BytecodeBuilder::DUMP_TYPES;
            if dump_constants {
                flags |= BytecodeBuilder::DUMP_CONSTANTS;
            }
            bcb.set_dump_flags(flags);
            bcb.set_dump_source(&source);
        } else if format == CompileFormat::Remarks {
            bcb.set_dump_flags(BytecodeBuilder::DUMP_SOURCE | BytecodeBuilder::DUMP_REMARKS);
            bcb.set_dump_source(&source);
        } else if format == CompileFormat::Codegen
            || format == CompileFormat::CodegenAsm
            || format == CompileFormat::CodegenIr
            || format == CompileFormat::CodegenVerbose
        {
            bcb.set_dump_flags(
                BytecodeBuilder::DUMP_CODE
                    | BytecodeBuilder::DUMP_SOURCE
                    | BytecodeBuilder::DUMP_LOCALS
                    | BytecodeBuilder::DUMP_REMARKS,
            );
            bcb.set_dump_source(&source);
        }

        stats.misc_time += record_delta_time(&mut currts);

        let mut allocator = Allocator::allocator();
        let mut names = AstNameTable::new(&mut allocator);
        let mut parse_options = ParseOptions::default();
        parse_options.store_cst_data = unsafe { globalOptions.parseCst };

        let parse_result = Parser::parse(
            source.as_str(),
            source.len(),
            &mut names,
            &mut allocator,
            parse_options,
        );

        if !parse_result.errors.is_empty() {
            for error in &parse_result.errors {
                report_error_c_char_luau_parse_error(name, error);
            }
            return false;
        }

        stats.lines += parse_result.lines;
        stats.parse_time += record_delta_time(&mut currts);

        if unsafe { globalOptions.onlyParse } {
            return true;
        }

        let compile_options = copts();
        compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
            &mut bcb,
            &parse_result,
            &mut names,
            &compile_options,
        );

        stats.bytecode += bcb.get_bytecode().len();
        stats.bytecode_instruction_count = bcb.get_total_instruction_count();
        stats.compile_time += record_delta_time(&mut currts);

        match format {
            CompileFormat::Text => {
                print!("{}", bcb.dump_everything());
            }
            CompileFormat::Remarks => {
                print!("{}", bcb.dump_source_remarks());
            }
            CompileFormat::Binary => {
                let _ = std::io::stdout().write_all(bcb.get_bytecode().as_bytes());
            }
            CompileFormat::Codegen
            | CompileFormat::CodegenAsm
            | CompileFormat::CodegenIr
            | CompileFormat::CodegenVerbose => {
                print!(
                    "{}",
                    get_codegen_assembly(name, bcb.get_bytecode(), options, &mut stats.lower_stats)
                );
            }
            CompileFormat::CodegenNull => {
                let assembly =
                    get_codegen_assembly(name, bcb.get_bytecode(), options, &mut stats.lower_stats);
                stats.codegen += assembly.len();
                stats.codegen_time += record_delta_time(&mut currts);
            }
            CompileFormat::Null => {}
        }

        true
    }));

    match result {
        Ok(success) => success,
        Err(payload) => {
            if let Some(error) = payload.downcast_ref::<CompileError>() {
                report_error_c_char_luau_compile_error(name, error);
                false
            } else {
                resume_unwind(payload);
            }
        }
    }
}
