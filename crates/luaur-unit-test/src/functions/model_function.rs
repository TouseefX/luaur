use core::ffi::c_char;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_ast::rtti::ast_node_as;
use luaur_compiler::functions::model_cost_cost_model_alt_b::model_cost_ast_node_ast_local_usize;

pub fn model_function(source: *const c_char) -> u64 {
    let source = unsafe { core::ffi::CStr::from_ptr(source) }
        .to_str()
        .expect("CostModel source must be valid UTF-8");

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let result = Parser::parse(
        source,
        source.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );
    assert!(
        result.errors.is_empty(),
        "unexpected parse error(s): {:?}",
        result.errors
    );
    assert!(!result.root.is_null());

    let first = unsafe { *(*result.root).body.data };
    let func = unsafe { ast_node_as::<AstStatFunction>(first as *mut AstNode) };
    assert!(!func.is_null());

    let function = unsafe { (*func).func };
    unsafe {
        model_cost_ast_node_ast_local_usize(
            (*function).body as *mut AstNode,
            (*function).args.data,
            (*function).args.size,
        )
    }
}
