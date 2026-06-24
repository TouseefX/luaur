extern crate alloc;

use crate::functions::dump_instruction::dump_instruction;
use crate::records::block::Block;
use crate::type_aliases::definition::Definition;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn dump_block(block: &Block, use_defs: &DenseHashMap<*mut AstExpr, *mut Definition>) -> String {
    let mut result = String::new();
    for inst in block.get_instructions() {
        result.push_str("  ");
        result.push_str(&dump_instruction(*inst, use_defs));
        result.push('\n');
    }
    result
}
