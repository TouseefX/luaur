use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parse_result::ParseResult;
use luaur_ast::type_aliases::cst_node_map::CstNodeMap;

/// `Reducer` is a native-only utility for reducing Luau source code while preserving a bug.
/// It is not portable to wasm32-unknown-unknown.
#[repr(C)]
#[derive(Debug)]
pub struct Reducer {
    pub(crate) allocator: Allocator,
    pub(crate) name_table: AstNameTable,
    pub(crate) parse_options: ParseOptions,
    pub(crate) parse_result: ParseResult,
    pub(crate) cst_node_map: CstNodeMap,
    pub(crate) root: *mut AstStatBlock,
    pub(crate) script_name: String,
    pub(crate) command: String,
    pub(crate) search_text: String,
    pub(crate) step: i32,
}

impl Reducer {
    pub fn new() -> Self {
        let mut allocator = Allocator::allocator();
        let name_table = AstNameTable::new(&mut allocator);
        let mut parse_options = ParseOptions::default();
        parse_options.capture_comments = true;
        parse_options.store_cst_data = true;

        Reducer {
            allocator,
            name_table,
            parse_options,
            parse_result: ParseResult {
                root: core::ptr::null_mut(),
                lines: 0,
                hotcomments: Vec::new(),
                errors: Vec::new(),
                comment_locations: Vec::new(),
                cst_node_map: CstNodeMap::new(core::ptr::null_mut()),
            },
            cst_node_map: CstNodeMap::new(core::ptr::null_mut()),
            root: core::ptr::null_mut(),
            script_name: String::new(),
            command: String::new(),
            search_text: String::new(),
            step: 0,
        }
    }
}

impl Default for Reducer {
    fn default() -> Self {
        Self::new()
    }
}
