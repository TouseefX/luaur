//! Source: `Analysis/src/FragmentAutocomplete.cpp:1000` (hand-ported)
use crate::records::fragment_parse_result::FragmentParseResult;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::position::Position;

/// C++ `parseFragment_DEPRECATED(...)` — the superseded fragment parser. The
/// project ports the current `parse_fragment` (`functions/parse_fragment.rs`);
/// this explicitly `_DEPRECATED` entry point has no Rust call site.
pub fn parse_fragment_deprecated(
    _root: *mut AstStatBlock,
    _names: *mut AstNameTable,
    _src: &str,
    _cursor_pos: &Position,
    _fragment_end_position: Option<Position>,
) -> Option<FragmentParseResult> {
    unimplemented!("C++ parseFragment_DEPRECATED; superseded by parse_fragment — no call site")
}
