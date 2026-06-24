use crate::functions::convert_require_suggestions_to_autocomplete_entry_map::convert_require_suggestions_to_autocomplete_entry_map;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_method_containing_extern_type::get_method_containing_extern_type;
use crate::functions::get_string_contents::get_string_contents;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::process_require_suggestions::process_require_suggestions;
use crate::records::file_resolver::FileResolver;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::string_completion_callback::StringCompletionCallback;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::{ast_node_as, ast_node_as_const, ast_node_is};

const K_REQUIRE_TAG_NAME: &str = "require";

pub fn autocomplete_string_params(
    module: &ModulePtr,
    nodes: &alloc::vec::Vec<*mut AstNode>,
    position: Position,
    file_resolver: *mut FileResolver,
    callback: StringCompletionCallback,
) -> Option<AutocompleteEntryMap> {
    if nodes.len() < 2 {
        return None;
    }

    let last = *nodes.last()?;
    if !unsafe { ast_node_is::<AstExprConstantString>(&*last) }
        && !is_simple_interpolated_string(last)
        && !unsafe { ast_node_is::<AstExprError>(&*last) }
    {
        return None;
    }

    if !unsafe { ast_node_is::<AstExprError>(&*last) } {
        let last_location = unsafe { (*last).location };
        if last_location.end == position || last_location.begin == position {
            return None;
        }
    }

    let candidate_node = nodes[nodes.len() - 2];
    let candidate = unsafe { ast_node_as::<AstExprCall>(candidate_node) };
    if candidate.is_null() {
        return None;
    }

    let candidate_ref = unsafe { &*candidate };

    if candidate_ref.args.size > 1 {
        let first_arg = unsafe { *candidate_ref.args.data };
        let first_arg_location = unsafe { (*first_arg).base.location };
        if !first_arg_location.contains(position) {
            return None;
        }
    }

    let it = module
        .ast_types
        .find(&(candidate_ref.func as *const AstExpr))?;
    let candidate_string = get_string_contents(last as *const AstNode);

    let mut perform_callback = |func_type: &FunctionType| -> Option<AutocompleteEntryMap> {
        for tag in &func_type.tags {
            if tag == K_REQUIRE_TAG_NAME && !file_resolver.is_null() {
                let suggestions = unsafe {
                    (*file_resolver)
                        .require_suggester
                        .as_ref()
                        .and_then(|suggester| {
                            suggester.get_require_suggestions_impl(&module.name, &candidate_string)
                        })
                };
                return convert_require_suggestions_to_autocomplete_entry_map(
                    process_require_suggestions(suggestions),
                );
            }

            if let Some(ret) = callback(
                tag.clone(),
                get_method_containing_extern_type(module, candidate_ref.func),
                candidate_string.clone(),
            ) {
                return Some(ret);
            }
        }

        None
    };

    let followed_id = unsafe { follow_type_id(*it) };
    let function_type = unsafe { get_type_id::<FunctionType>(followed_id) };
    if !function_type.is_null() {
        return perform_callback(unsafe { &*function_type });
    }

    let intersect = unsafe { get_type_id::<IntersectionType>(followed_id) };
    if !intersect.is_null() {
        for part in unsafe { &(*intersect).parts } {
            let part = unsafe { follow_type_id(*part) };
            let candidate_function_type = unsafe { get_type_id::<FunctionType>(part) };
            if !candidate_function_type.is_null() {
                if let Some(ret) = perform_callback(unsafe { &*candidate_function_type }) {
                    return Some(ret);
                }
            }
        }
    }

    None
}

fn is_simple_interpolated_string(node: *const AstNode) -> bool {
    let interp_string = unsafe { ast_node_as_const::<AstExprInterpString>(node) };
    !interp_string.is_null() && unsafe { (*interp_string).expressions.size == 0 }
}
