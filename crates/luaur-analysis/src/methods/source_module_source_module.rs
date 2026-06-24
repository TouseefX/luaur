use crate::records::source_module::SourceModule;
use alloc::sync::Arc;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;

impl SourceModule {
    pub fn source_module() -> Self {
        let mut allocator = Arc::new(Allocator::allocator());
        let names = AstNameTable::new(
            Arc::get_mut(&mut allocator).expect("fresh SourceModule allocator must be unique"),
        );
        Self {
            name: String::new(),
            human_readable_name: String::new(),
            r#type: crate::records::source_code::SourceCode::None,
            environment_name: None,
            cyclic: false,
            allocator,
            names: Arc::new(names),
            parse_errors: Vec::new(),
            root: core::ptr::null_mut(),
            mode: None,
            hotcomments: Vec::new(),
            comment_locations: Vec::new(),
        }
    }
}
