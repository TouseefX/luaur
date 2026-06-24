use crate::functions::check_overloaded_documentation_symbol::check_overloaded_documentation_symbol;
use crate::functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position_source_module_position_bool;
use crate::functions::find_binding_at_position::find_binding_at_position;
use crate::functions::find_type_at_position::find_type_at_position;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::binding::Binding;
use crate::records::extern_type::ExternType;
use crate::records::module::Module;
use crate::records::primitive_type::PrimitiveType;
use crate::records::source_module::SourceModule;
use crate::records::table_type::TableType;
use crate::type_aliases::documentation_symbol::DocumentationSymbol;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_as;

pub fn get_documentation_symbol_at_position(
    source: &SourceModule,
    module: &Module,
    position: Position,
) -> Option<DocumentationSymbol> {
    let ancestry =
        find_ast_ancestry_of_position_source_module_position_bool(source, position, false);

    // C++: ancestry[...]->asExpr() — base-class downcast, not concrete RTTI.
    let target_expr = if ancestry.len() >= 1 {
        unsafe { (*(ancestry[ancestry.len() - 1] as *mut AstNode)).as_expr() }
    } else {
        std::ptr::null_mut()
    };

    let parent_expr = if ancestry.len() >= 2 {
        unsafe { (*(ancestry[ancestry.len() - 2] as *mut AstNode)).as_expr() }
    } else {
        std::ptr::null_mut()
    };

    if !target_expr.is_null() {
        if unsafe { ast_node_as::<AstExprIndexName>(target_expr as *mut AstNode) }.is_null()
            == false
        {
            let index_name =
                unsafe { ast_node_as::<AstExprIndexName>(target_expr as *mut AstNode) };
            // C++: module.astTypes.find(indexName->expr) — key is *const AstExpr.
            let it = module
                .ast_types
                .find(&(unsafe { (*index_name).expr } as *const AstExpr));

            if let Some(parent_ty) = it {
                let follow_ty = unsafe { follow_type_id(*parent_ty) };

                if unsafe { get_type_id::<TableType>(follow_ty) }.is_null() == false {
                    let ttv = unsafe { &*get_type_id::<TableType>(follow_ty) };
                    let index_value = unsafe { (*index_name).index.value };
                    // C++: ttv->props.find(indexName->index.value) — props keyed by std::string.
                    let index_key = unsafe {
                        core::ffi::CStr::from_ptr(index_value)
                            .to_string_lossy()
                            .into_owned()
                    };
                    if let Some(prop_it) = ttv.props.get(&index_key) {
                        if let Some(ty) = unsafe { (*prop_it).read_ty } {
                            return check_overloaded_documentation_symbol(
                                module,
                                ty,
                                parent_expr as *const AstExpr,
                                unsafe { (*prop_it).documentation_symbol.clone() },
                            );
                        }
                    }
                } else if unsafe { get_type_id::<ExternType>(follow_ty) }.is_null() == false {
                    let mut etv_ptr = unsafe { get_type_id::<ExternType>(follow_ty) };
                    while !etv_ptr.is_null() {
                        let etv = unsafe { &*etv_ptr };
                        let index_value = unsafe { (*index_name).index.value };
                        // C++: etv->props.find(indexName->index.value) — props keyed by std::string.
                        let index_key = unsafe {
                            core::ffi::CStr::from_ptr(index_value)
                                .to_string_lossy()
                                .into_owned()
                        };
                        if let Some(prop_it) = etv.props.get(&index_key) {
                            if let Some(ty) = unsafe { (*prop_it).read_ty } {
                                return check_overloaded_documentation_symbol(
                                    module,
                                    ty,
                                    parent_expr as *const AstExpr,
                                    unsafe { (*prop_it).documentation_symbol.clone() },
                                );
                            }
                        }

                        etv_ptr = if let Some(parent_ty) = etv.parent {
                            unsafe { get_type_id::<ExternType>(follow_type_id(parent_ty)) }
                        } else {
                            std::ptr::null_mut()
                        };
                    }
                } else if unsafe { get_type_id::<PrimitiveType>(follow_ty) }.is_null() == false {
                    let ptv = unsafe { &*get_type_id::<PrimitiveType>(follow_ty) };

                    if let Some(metatable_ty) = ptv.metatable {
                        if unsafe { get_type_id::<TableType>(metatable_ty) }.is_null() == false {
                            let mtable = unsafe { &*get_type_id::<TableType>(metatable_ty) };
                            let index = unsafe { (*index_name).index };
                            return crate::functions::get_metatable_documentation::get_metatable_documentation(module, parent_expr as *const AstExpr, mtable, &index);
                        }
                    }
                }
            }
        } else if unsafe { ast_node_as::<AstExprFunction>(target_expr as *mut AstNode) }.is_null()
            == false
        {
            if !parent_expr.is_null()
                && unsafe { ast_node_as::<AstExprCall>(parent_expr as *mut AstNode) }.is_null()
                    == false
            {
                let call = unsafe { ast_node_as::<AstExprCall>(parent_expr as *mut AstNode) };

                if let Some(parent_symbol) = get_documentation_symbol_at_position(
                    source,
                    module,
                    unsafe { (*(*call).func).base.location }.begin,
                ) {
                    for i in 0..unsafe { (*call).args.size } as usize {
                        let call_arg = unsafe { *((*call).args.data.add(i)) };

                        if call_arg == target_expr {
                            let fn_symbol = format!("{}/param/{}", parent_symbol, i);

                            let fn_node = unsafe {
                                ast_node_as::<AstExprFunction>(target_expr as *mut AstNode)
                            };
                            for j in 0..unsafe { (*fn_node).args.size } as usize {
                                let fn_arg = unsafe { *((*fn_node).args.data.add(j)) };

                                if unsafe { (*fn_arg).location }.contains(position) {
                                    return Some(format!("{}/param/{}", fn_symbol, j));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(binding) = find_binding_at_position(module, source, position) {
        return check_overloaded_documentation_symbol(
            module,
            binding.type_id,
            parent_expr as *const AstExpr,
            binding.documentation_symbol,
        );
    }

    if let Some(ty) = find_type_at_position(module, source, position) {
        let ty_ptr = unsafe { &*ty };
        if let Some(doc_symbol) = ty_ptr.documentation_symbol.clone() {
            return Some(doc_symbol);
        }
    }

    None
}
