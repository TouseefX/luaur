use crate::functions::find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position;
use crate::functions::find_ancestry_for_fragment_parse::find_ancestry_for_fragment_parse;
use crate::functions::get_document_offsets::get_document_offsets;
use crate::records::fragment_parse_result::FragmentParseResult;
use alloc::boxed::Box;
use alloc::string::String;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::fragment_parse_resume_settings::FragmentParseResumeSettings;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_ast::records::position::Position;

pub fn parse_fragment(
    stale: *mut AstStatBlock,
    most_recent_parse: *mut AstStatBlock,
    names: *mut AstNameTable,
    src: &str,
    cursor_pos: &Position,
    fragment_end_position: Option<Position>,
) -> Option<FragmentParseResult> {
    if most_recent_parse.is_null() {
        return None;
    }

    let result = find_ancestry_for_fragment_parse(stale, *cursor_pos, most_recent_parse);
    let mut nearest_statement = result.nearestStatement;

    let start_pos = result.fragmentSelectionRegion.begin;
    let end_pos = fragment_end_position.unwrap_or(result.fragmentSelectionRegion.end);
    let (offset_start, parse_length) = get_document_offsets(src, &start_pos, &end_pos);
    let fragment_source = &src[offset_start..offset_start + parse_length];

    let mut fragment_alloc = Box::new(Allocator::allocator());
    let mut parse_options = ParseOptions::default();
    parse_options.allow_declaration_syntax = false;
    parse_options.capture_comments = true;
    parse_options.parse_fragment = Some(FragmentParseResumeSettings::new(
        result.localMap,
        result.localStack,
        start_pos,
    ));

    let parse_result = unsafe {
        Parser::parse(
            fragment_source,
            parse_length,
            &mut *names,
            &mut *fragment_alloc,
            parse_options,
        )
    };

    if parse_result.root.is_null() {
        return None;
    }

    let mut fabricated_ancestry =
        find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
            unsafe { &mut *most_recent_parse },
            *cursor_pos,
        );
    let fragment_ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
        unsafe { &mut *parse_result.root },
        *cursor_pos,
    );

    let mut back = fabricated_ancestry.len();
    for fragment_node in fragment_ancestry.iter().rev() {
        if back == 0 {
            break;
        }

        back -= 1;
        let fabricated_node = fabricated_ancestry[back];
        if !fragment_node.is_null()
            && !fabricated_node.is_null()
            && unsafe { (**fragment_node).class_index == (*fabricated_node).class_index }
        {
            fabricated_ancestry[back] = *fragment_node;
        }
    }

    if nearest_statement.is_null() {
        nearest_statement = parse_result.root as *mut AstStat;
    }

    let scope_pos = if result.parentBlock.is_null() {
        Position { line: 0, column: 0 }
    } else {
        unsafe { (*result.parentBlock).base.base.location.begin }
    };

    Some(FragmentParseResult {
        fragment_to_parse: String::from(fragment_source),
        root: parse_result.root,
        ancestry: fabricated_ancestry,
        nearest_statement,
        comment_locations: parse_result.comment_locations,
        alloc: fragment_alloc,
        scope_pos,
    })
}
