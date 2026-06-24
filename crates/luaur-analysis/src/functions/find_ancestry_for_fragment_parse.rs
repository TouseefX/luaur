use alloc::vec::Vec;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

use crate::functions::find_ancestry_at_position_for_autocomplete_ast_query_alt_b::find_ancestry_at_position_for_autocomplete_ast_stat_block_position;
use crate::functions::get_fragment_region_with_block_diff::get_fragment_region_with_block_diff;
use crate::records::fragment_autocomplete_ancestry_result::FragmentAutocompleteAncestryResult;

pub fn find_ancestry_for_fragment_parse(
    stale: *mut AstStatBlock,
    cursor_pos: Position,
    last_good_parse: *mut AstStatBlock,
) -> FragmentAutocompleteAncestryResult {
    // the freshest ast can sometimes be null if the parse was bad.
    if last_good_parse.is_null() {
        return FragmentAutocompleteAncestryResult {
            localMap: DenseHashMap::new(AstName::new()),
            localStack: Vec::new(),
            ancestry: Vec::new(),
            nearestStatement: core::ptr::null_mut(),
            parentBlock: core::ptr::null_mut(),
            fragmentSelectionRegion: Location::new(Position::missing(), Position::missing()),
        };
    }

    let region = get_fragment_region_with_block_diff(stale, last_good_parse, &cursor_pos);
    let ancestry = find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
        unsafe { &mut *stale },
        cursor_pos,
    );

    LUAU_ASSERT!(ancestry.len() >= 1);

    // We should only pick up locals that are before the region
    let mut local_map: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::new(AstName::new());
    let mut local_stack: Vec<*mut AstLocal> = Vec::new();

    for node in &ancestry {
        let node = *node;

        let block = unsafe { ast_node_as::<AstStatBlock>(node) };
        if !block.is_null() {
            for stat in unsafe { &(*block).body } {
                let stat = *stat;
                if unsafe { (*stat).base.location.begin } < region.fragment_location.begin {
                    // This statement precedes the current one
                    let stat_loc = unsafe { ast_node_as::<AstStatLocal>(stat as *mut AstNode) };
                    let loc_fun =
                        unsafe { ast_node_as::<AstStatLocalFunction>(stat as *mut AstNode) };
                    let glob_fun = unsafe { ast_node_as::<AstStatFunction>(stat as *mut AstNode) };
                    let type_fun =
                        unsafe { ast_node_as::<AstStatTypeFunction>(stat as *mut AstNode) };
                    let for_l = unsafe { ast_node_as::<AstStatFor>(stat as *mut AstNode) };
                    let for_in = unsafe { ast_node_as::<AstStatForIn>(stat as *mut AstNode) };
                    let class_decl = unsafe { ast_node_as::<AstStatClass>(stat as *mut AstNode) };

                    if !stat_loc.is_null() {
                        for v in unsafe { &(*stat_loc).vars } {
                            let v = *v;
                            local_stack.push(v);
                            *local_map.get_or_insert(unsafe { (*v).name }) = v;
                        }
                    } else if !loc_fun.is_null() {
                        let name = unsafe { (*loc_fun).name };
                        local_stack.push(name);
                        *local_map.get_or_insert(unsafe { (*name).name }) = name;
                        if unsafe { (*loc_fun).base.base.location.contains(cursor_pos) } {
                            let func = unsafe { (*loc_fun).func };
                            for loc in unsafe { &(*func).args } {
                                let loc = *loc;
                                local_stack.push(loc);
                                *local_map.get_or_insert(unsafe { (*loc).name }) = loc;
                            }
                        }
                    } else if !glob_fun.is_null() {
                        if unsafe { (*glob_fun).base.base.location.contains(cursor_pos) } {
                            let func = unsafe { (*glob_fun).func };
                            let local = unsafe { (*func).self_ };
                            if !local.is_null() {
                                local_stack.push(local);
                                *local_map.get_or_insert(unsafe { (*local).name }) = local;
                            }

                            for loc in unsafe { &(*func).args } {
                                let loc = *loc;
                                local_stack.push(loc);
                                *local_map.get_or_insert(unsafe { (*loc).name }) = loc;
                            }
                        }
                    } else if !type_fun.is_null() {
                        if unsafe { (*type_fun).base.base.location.contains(cursor_pos) } {
                            let body = unsafe { (*type_fun).body };
                            for loc in unsafe { &(*body).args } {
                                let loc = *loc;
                                local_stack.push(loc);
                                *local_map.get_or_insert(unsafe { (*loc).name }) = loc;
                            }
                        }
                    } else if !for_l.is_null() {
                        let var = unsafe { (*for_l).var };
                        if !var.is_null()
                            && unsafe { (*var).location.begin } < region.fragment_location.begin
                        {
                            local_stack.push(var);
                            *local_map.get_or_insert(unsafe { (*var).name }) = var;
                        }
                    } else if !for_in.is_null() {
                        for var in unsafe { &(*for_in).vars } {
                            let var = *var;
                            if unsafe { (*var).location.begin } < region.fragment_location.begin {
                                local_stack.push(var);
                                *local_map.get_or_insert(unsafe { (*var).name }) = var;
                            }
                        }
                    } else if !class_decl.is_null() {
                        // We need to include the class name as part of the
                        // locals so that within the fragment the class name
                        // is defined.
                        let name = unsafe { (*class_decl).name };
                        local_stack.push(name);
                        *local_map.get_or_insert(unsafe { (*name).name }) = name;
                        if unsafe { (*class_decl).base.base.location.containsClosed(cursor_pos) } {
                            let mut current_method: *mut AstExprFunction = core::ptr::null_mut();
                            for decl in unsafe { &(*class_decl).members } {
                                if let Some(method) = decl.get_if_1() {
                                    if unsafe {
                                        (*(*method.function).body).base.base.location.begin
                                    } < cursor_pos
                                    {
                                        current_method = method.function;
                                    }
                                }
                            }
                            if !current_method.is_null() {
                                for v in unsafe { &(*current_method).args } {
                                    let v = *v;
                                    local_stack.push(v);
                                    *local_map.get_or_insert(unsafe { (*v).name }) = v;
                                }
                            }
                        }
                    }
                }
            }
        }

        let expr_func = unsafe { ast_node_as::<AstExprFunction>(node) };
        if !expr_func.is_null() {
            if unsafe { (*expr_func).base.base.location.contains(cursor_pos) } {
                for v in unsafe { &(*expr_func).args } {
                    let v = *v;
                    local_stack.push(v);
                    *local_map.get_or_insert(unsafe { (*v).name }) = v;
                }
            }
        }
    }

    FragmentAutocompleteAncestryResult {
        localMap: local_map,
        localStack: local_stack,
        ancestry,
        nearestStatement: region.nearest_statement,
        parentBlock: region.parent_block,
        fragmentSelectionRegion: region.fragment_location,
    }
}
