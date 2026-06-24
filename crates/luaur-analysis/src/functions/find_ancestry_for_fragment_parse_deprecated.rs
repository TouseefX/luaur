use alloc::vec::Vec;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;
use luaur_ast::rtti::ast_node_is;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

use crate::functions::find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position;
use crate::functions::stat_is_before_pos::stat_is_before_pos;
use crate::records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult;

pub fn find_ancestry_for_fragment_parse_deprecated(
    root: *mut AstStatBlock,
    cursor_pos: &Position,
) -> FragmentAutocompleteAncestryResult {
    let ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
        unsafe { &mut *root },
        *cursor_pos,
    );
    LUAU_ASSERT!(ancestry.len() >= 1);

    let mut local_map = DenseHashMap::new(AstName::new());
    let mut local_stack: Vec<*mut AstLocal> = Vec::new();
    let mut nearest_statement: *mut AstStat = std::ptr::null_mut();

    for node in &ancestry {
        if unsafe { ast_node_is::<AstStatBlock>(&*(*node)) } {
            let block = unsafe { ast_node_as::<AstStatBlock>(*node as *mut AstNode) };
            let body = unsafe { &(*block).body };
            for i in 0..body.size as usize {
                let stat = unsafe { *body.data.add(i) };
                let stat_location = unsafe { (*stat).base.location };
                if stat_location.begin <= *cursor_pos {
                    nearest_statement = stat;
                }
            }
        }
    }

    if nearest_statement.is_null() {
        let first: *mut AstNode = ancestry[0];
        nearest_statement = unsafe { (*first).as_stat() };
    }
    LUAU_ASSERT!(!nearest_statement.is_null());

    let nearest_statement_begin = unsafe { (*nearest_statement).base.location.begin };

    for node in &ancestry {
        if unsafe { ast_node_is::<AstStatBlock>(&*(*node)) } {
            let block = unsafe { ast_node_as::<AstStatBlock>(*node as *mut AstNode) };
            let body = unsafe { &(*block).body };
            for i in 0..body.size as usize {
                let stat = unsafe { *body.data.add(i) };
                if stat_is_before_pos(
                    unsafe { &*(stat as *mut AstNode) },
                    &nearest_statement_begin,
                ) {
                    if unsafe { ast_node_is::<AstStatLocal>(&*(stat as *mut AstNode)) } {
                        let stat_local =
                            unsafe { ast_node_as::<AstStatLocal>(stat as *mut AstNode) };
                        let vars = unsafe { &(*stat_local).vars };
                        for j in 0..vars.size as usize {
                            let var = unsafe { *vars.data.add(j) };
                            local_stack.push(var);
                            let var_name = unsafe { (*var).name };
                            *local_map.get_or_insert(var_name) = var;
                        }
                    } else if unsafe {
                        ast_node_is::<AstStatLocalFunction>(&*(stat as *mut AstNode))
                    } {
                        let stat_local_function =
                            unsafe { ast_node_as::<AstStatLocalFunction>(stat as *mut AstNode) };
                        let name = unsafe { (*stat_local_function).name };
                        local_stack.push(name);
                        let name_name = unsafe { (*name).name };
                        *local_map.get_or_insert(name_name) = name;
                        let stat_location = unsafe { (*stat).base.location };
                        if stat_location.contains(*cursor_pos) {
                            let func = unsafe { (*stat_local_function).func };
                            let args = unsafe { (*func).args };
                            for j in 0..args.size as usize {
                                let loc = unsafe { *args.data.add(j) };
                                local_stack.push(loc);
                                let loc_name = unsafe { (*loc).name };
                                *local_map.get_or_insert(loc_name) = loc;
                            }
                        }
                    } else if unsafe { ast_node_is::<AstStatFunction>(&*(stat as *mut AstNode)) } {
                        let stat_function =
                            unsafe { ast_node_as::<AstStatFunction>(stat as *mut AstNode) };
                        let stat_location = unsafe { (*stat).base.location };
                        if stat_location.contains(*cursor_pos) {
                            let func = unsafe { (*stat_function).func };
                            let args = unsafe { (*func).args };
                            for j in 0..args.size as usize {
                                let loc = unsafe { *args.data.add(j) };
                                local_stack.push(loc);
                                let loc_name = unsafe { (*loc).name };
                                *local_map.get_or_insert(loc_name) = loc;
                            }
                        }
                    } else if unsafe {
                        ast_node_is::<AstStatTypeFunction>(&*(stat as *mut AstNode))
                    } {
                        let stat_type_function =
                            unsafe { ast_node_as::<AstStatTypeFunction>(stat as *mut AstNode) };
                        let stat_location = unsafe { (*stat).base.location };
                        if stat_location.contains(*cursor_pos) {
                            let body = unsafe { (*stat_type_function).body };
                            let args = unsafe { (*body).args };
                            for j in 0..args.size as usize {
                                let loc = unsafe { *args.data.add(j) };
                                local_stack.push(loc);
                                let loc_name = unsafe { (*loc).name };
                                *local_map.get_or_insert(loc_name) = loc;
                            }
                        }
                    }
                }
            }
        }

        if unsafe { ast_node_is::<AstExprFunction>(&*(*node)) } {
            let expr_func = unsafe { ast_node_as::<AstExprFunction>(*node as *mut AstNode) };
            let expr_location = unsafe { (*expr_func).base.base.location };
            if expr_location.contains(*cursor_pos) {
                let args = unsafe { (*expr_func).args };
                for j in 0..args.size as usize {
                    let v = unsafe { *args.data.add(j) };
                    local_stack.push(v);
                    let v_name = unsafe { (*v).name };
                    *local_map.get_or_insert(v_name) = v;
                }
            }
        }
    }

    // C++ aggregate-initializes only the first four members; parentBlock and
    // fragmentSelectionRegion are value-initialized (null / default Location).
    FragmentAutocompleteAncestryResult {
        localMap: local_map,
        localStack: local_stack,
        ancestry,
        nearestStatement: nearest_statement,
        parentBlock: core::ptr::null_mut(),
        fragmentSelectionRegion: Location::default(),
    }
}
