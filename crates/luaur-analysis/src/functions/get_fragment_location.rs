use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;

use crate::functions::get_function_declaration_extents::get_function_declaration_extents;
use crate::functions::get_nearest_if_to_cursor::get_nearest_if_to_cursor;

pub fn get_fragment_location(
    nearest_statement: *mut AstStat,
    cursor_position: &Position,
) -> Location {
    let empty = Location::new(*cursor_position, *cursor_position);

    if nearest_statement.is_null() {
        return empty;
    }

    let non_empty = Location::new(
        unsafe { (*nearest_statement).base.location.begin },
        *cursor_position,
    );

    // If your sibling is a do block, do nothing
    if unsafe {
        ast_node_as::<AstStatBlock>(nearest_statement as *mut luaur_ast::records::ast_node::AstNode)
    }
    .is_null()
        == false
    {
        return empty;
    }

    // Handle AstStatFunction
    if let Some(stat_func) = unsafe {
        ast_node_as::<AstStatFunction>(
            nearest_statement as *mut luaur_ast::records::ast_node::AstNode,
        )
        .as_mut()
    } {
        let func = unsafe { (*stat_func).func };
        let name = unsafe { (*stat_func).name };
        let local = std::ptr::null_mut();
        let loc = get_function_declaration_extents(func, name, local);

        if loc.containsClosed(*cursor_position) {
            return non_empty;
        } else {
            let body = unsafe { (*func).body };
            let body_location = unsafe { (*body).base.base.location };
            if body_location.containsClosed(*cursor_position)
                || unsafe { (*stat_func).base.base.location.end } <= *cursor_position
            {
                return empty;
            } else if unsafe { (*func).base.base.location }.contains(*cursor_position) {
                return non_empty;
            }
        }
    }

    // Handle AstStatLocalFunction
    if let Some(stat_local_func) = unsafe {
        ast_node_as::<AstStatLocalFunction>(
            nearest_statement as *mut luaur_ast::records::ast_node::AstNode,
        )
        .as_mut()
    } {
        let func = unsafe { (*stat_local_func).func };
        let name = unsafe { (*stat_local_func).name };
        let global_func = std::ptr::null_mut();
        let loc = get_function_declaration_extents(func, global_func, name);

        if loc.containsClosed(*cursor_position) {
            return non_empty;
        } else {
            let body = unsafe { (*func).body };
            let body_location = unsafe { (*body).base.base.location };
            if body_location.containsClosed(*cursor_position)
                || unsafe { (*stat_local_func).base.base.location.end } <= *cursor_position
            {
                return empty;
            } else if unsafe { (*func).base.base.location }.contains(*cursor_position) {
                return non_empty;
            }
        }
    }

    // Handle AstStatWhile
    if let Some(stat_while) = unsafe {
        ast_node_as::<AstStatWhile>(nearest_statement as *mut luaur_ast::records::ast_node::AstNode)
            .as_mut()
    } {
        if unsafe { !(*stat_while).has_do } {
            return non_empty;
        } else {
            return empty;
        }
    }

    // Handle AstStatFor
    if let Some(stat_for) = unsafe {
        ast_node_as::<AstStatFor>(nearest_statement as *mut luaur_ast::records::ast_node::AstNode)
            .as_mut()
    } {
        if let Some(step) = unsafe { (*stat_for).step.as_mut() } {
            let step_location = unsafe { (*step).base.location };
            if step_location.containsClosed(*cursor_position) {
                return Location::new(step_location.begin, *cursor_position);
            }
        }
        if let Some(to) = unsafe { (*stat_for).to.as_mut() } {
            let to_location = unsafe { (*to).base.location };
            if to_location.containsClosed(*cursor_position) {
                return Location::new(to_location.begin, *cursor_position);
            }
        }
        if let Some(from) = unsafe { (*stat_for).from.as_mut() } {
            let from_location = unsafe { (*from).base.location };
            if from_location.containsClosed(*cursor_position) {
                return Location::new(from_location.begin, *cursor_position);
            }
        }

        if unsafe { !(*stat_for).has_do } {
            return non_empty;
        } else {
            let completeable_extents =
                Location::new(unsafe { (*stat_for).base.base.location.begin }, unsafe {
                    (*stat_for).do_location.begin
                });
            if completeable_extents.containsClosed(*cursor_position) {
                return non_empty;
            }
            return empty;
        }
    }

    // Handle AstStatForIn
    if let Some(stat_for_in) = unsafe {
        ast_node_as::<AstStatForIn>(nearest_statement as *mut luaur_ast::records::ast_node::AstNode)
            .as_mut()
    } {
        if unsafe { !(*stat_for_in).has_do } {
            return non_empty;
        } else {
            let completeable_extents =
                Location::new(unsafe { (*stat_for_in).base.base.location.begin }, unsafe {
                    (*stat_for_in).do_location.begin
                });
            if completeable_extents.containsClosed(*cursor_position) {
                if unsafe { !(*stat_for_in).has_in } {
                    return non_empty;
                } else {
                    // [for ... in ... do] - the cursor can either be between [for ... in] or [in ... do]
                    if *cursor_position < unsafe { (*stat_for_in).in_location.begin } {
                        return non_empty;
                    } else {
                        return Location::new(
                            unsafe { (*stat_for_in).in_location.begin },
                            *cursor_position,
                        );
                    }
                }
            }
            return empty;
        }
    }

    // Handle AstStatIf
    if let Some(if_stmt) =
        unsafe { get_nearest_if_to_cursor(nearest_statement, cursor_position).as_mut() }
    {
        let condition_location = unsafe { (*if_stmt).condition };
        let condition_extents = Location::new(
            unsafe { (*condition_location).base.location.begin },
            unsafe { (*condition_location).base.location.end },
        );

        if condition_extents.containsClosed(*cursor_position)
            || unsafe { (*if_stmt).then_location.is_none() }
        {
            // CLI-152249 - the condition parse location can sometimes be after the body of the if
            // statement. This is a bug that results returning locations like {3,0 - 2,0} which is
            // wrong.
            if unsafe { (*condition_location).base.location.begin } > *cursor_position {
                return empty;
            }
            return Location::new(
                unsafe { (*condition_location).base.location.begin },
                *cursor_position,
            );
        } else if unsafe { (*(*if_stmt).thenbody).base.base.location }
            .containsClosed(*cursor_position)
        {
            return empty;
        } else if let Some(else_body) = unsafe { (*if_stmt).elsebody.as_mut() } {
            let else_body_ptr: *mut AstStat = else_body;
            if let Some(else_if) = unsafe {
                ast_node_as::<AstStatIf>(
                    else_body_ptr as *mut luaur_ast::records::ast_node::AstNode,
                )
                .as_mut()
            } {
                let else_if_condition_extents =
                    Location::new(unsafe { (*else_if).base.base.location.begin }, unsafe {
                        (*(*else_if).condition).base.location.end
                    });
                if else_if_condition_extents.containsClosed(*cursor_position) {
                    return Location::new(
                        unsafe { (*(*else_if).condition).base.location.begin },
                        *cursor_position,
                    );
                }
                if unsafe { (*(*else_if).thenbody).has_end } {
                    return empty;
                } else {
                    return Location::new(
                        unsafe { (*else_body_ptr).base.location.begin },
                        *cursor_position,
                    );
                }
            }
            return empty;
        }
    }

    non_empty
}
