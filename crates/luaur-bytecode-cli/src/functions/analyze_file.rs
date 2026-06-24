use alloc::string::String;
use alloc::vec::Vec;
use std::ffi::CString;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::parse_error::ParseError;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_cli_lib::functions::read_file::read_file;
use luaur_code_gen::functions::summarize_bytecode::summarize_bytecode;
use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
use luaur_compiler::functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options;
use luaur_compiler::records::compile_error::CompileError;
use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::copts::copts;

struct LuaStateGuard(*mut lua_State);

impl Drop for LuaStateGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                lua_close(self.0);
            }
        }
    }
}

fn nul_terminated(value: &str) -> Option<String> {
    CString::new(value)
        .ok()
        .and_then(|value| String::from_utf8(value.as_bytes_with_nul().to_vec()).ok())
}

fn report_parse_error(name: &str, error: &ParseError) {
    let location = error.get_location();
    eprintln!(
        "{}({},{}): SyntaxError: {}",
        name,
        location.begin.line + 1,
        location.begin.column + 1,
        error.what()
    );
}

fn report_compile_error(name: &str, error: &CompileError) {
    let location = error.get_location();
    eprintln!(
        "{}({},{}): CompileError: {}",
        name,
        location.begin.line + 1,
        location.begin.column + 1,
        error
    );
}

pub fn analyze_file(
    name: &str,
    nesting_limit: u32,
    summaries: &mut Vec<FunctionBytecodeSummary>,
) -> bool {
    let Some(name_with_nul) = nul_terminated(name) else {
        eprintln!("Error opening {}", name);
        return false;
    };

    let Some(source) = read_file(&name_with_nul) else {
        eprintln!("Error opening {}", name);
        return false;
    };

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let parse_options = ParseOptions::default();
    let parse_result = Parser::parse(
        source.as_str(),
        source.len(),
        &mut names,
        &mut allocator,
        parse_options,
    );

    if !parse_result.errors.is_empty() {
        for error in &parse_result.errors {
            report_parse_error(name, error);
        }

        return false;
    }

    let mut bcb = BytecodeBuilder::new(None);
    let options = copts();
    let compile_result = catch_unwind(AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
            &mut bcb,
            &parse_result,
            &mut names,
            &options,
        );
    }));

    if let Err(payload) = compile_result {
        if let Some(error) = payload.downcast_ref::<CompileError>() {
            report_compile_error(name, error);
            return false;
        }

        resume_unwind(payload);
    }

    let bytecode = bcb.get_bytecode();
    let global_state = LuaStateGuard(lua_l_newstate());
    let l = global_state.0;

    if unsafe {
        luau_load(
            l,
            name_with_nul.as_ptr() as *const core::ffi::c_char,
            bytecode.as_ptr() as *const core::ffi::c_char,
            bytecode.len(),
            0,
        )
    } == 0
    {
        *summaries = summarize_bytecode(l, -1, nesting_limit);
        true
    } else {
        eprintln!("Error loading bytecode {}", name);
        false
    }
}
