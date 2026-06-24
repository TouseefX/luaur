//! `void synthesizeExportReturn(NotNull<BuiltinTypes> builtinTypes, NotNull<Module> module)`.
//! Reference: `Module.cpp:361-467`.

use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::module::Module;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::records::table_type::TableType;
use crate::records::type_pack::TypePack;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec;
use core::ffi::CStr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::rtti::ast_node_as;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

fn key_of(name: luaur_ast::records::ast_name::AstName) -> String {
    if name.value.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() }
    }
}

/// C++ `Property(TypeId readTy)` — the single-argument constructor sets
/// `readTy == writeTy` (a read-write property). Reference: `Type.h` Property ctor.
fn prop_from_ty(ty: TypeId) -> Property {
    Property {
        read_ty: Some(ty),
        write_ty: Some(ty),
        ..Property::default()
    }
}

pub fn synthesize_export_return(builtin_types: *mut BuiltinTypes, module: *mut Module) {
    let module_ref = unsafe { &mut *module };
    LUAU_ASSERT!(!module_ref.root.is_null());

    let module_scope = module_ref.get_module_scope();
    let module_scope_ptr = alloc::sync::Arc::as_ptr(&module_scope) as *mut Scope;
    let mut props: Props = Props::new();

    let lookup_exported_binding_type = |local: *mut AstLocal| -> TypeId {
        let scope =
            unsafe { (*module_scope_ptr).find_narrowest_scope_containing((*local).location) };

        if let Some((binding, _scope)) =
            unsafe { (*scope).lookup_ex_symbol(Symbol::from_local(local)) }
        {
            return unsafe { follow_type_id((*binding).type_id) };
        }

        unsafe { (*builtin_types).errorType }
    };

    let lookup_expr_type = |expr: *mut AstExpr| -> TypeId {
        if let Some(ty) = unsafe { (*module).ast_types.find(&(expr as *const AstExpr)) } {
            return unsafe { follow_type_id(*ty) };
        }

        unsafe { (*builtin_types).errorType }
    };

    let mut exported_locals: DenseHashSet<*mut AstLocal> = DenseHashSet::new(core::ptr::null_mut());

    let body = unsafe { &(*module_ref.root).body };
    for i in 0..body.size {
        let statement = unsafe { *body.data.add(i) };
        let node = statement as *mut AstNode;

        let local_stat = unsafe { ast_node_as::<AstStatLocal>(node) };
        let local_function = unsafe { ast_node_as::<AstStatLocalFunction>(node) };
        let assign = unsafe { ast_node_as::<AstStatAssign>(node) };
        let func_stat = unsafe { ast_node_as::<AstStatFunction>(node) };

        if !local_stat.is_null() {
            let local_stat = unsafe { &*local_stat };
            if !local_stat.is_exported {
                continue;
            }

            for i in 0..local_stat.vars.size {
                let local = unsafe { *local_stat.vars.data.add(i) };
                exported_locals.insert(local);

                let key = key_of(unsafe { (*local).name });

                if local_stat.vars.size != local_stat.values.size || i >= local_stat.values.size {
                    props.insert(
                        key.clone(),
                        prop_from_ty(lookup_exported_binding_type(local)),
                    );
                } else {
                    let value = unsafe { *local_stat.values.data.add(i) };
                    props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
                }

                props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
            }
        } else if !local_function.is_null() {
            let local_function = unsafe { &*local_function };
            if unsafe { !(*local_function.name).is_exported } {
                continue;
            }

            let key = key_of(unsafe { (*local_function.name).name });
            props.insert(
                key.clone(),
                Property::readonly(lookup_exported_binding_type(local_function.name)),
            );
            props.get_mut(&key).unwrap().location =
                Some(unsafe { (*local_function.name).location });
        } else if !assign.is_null() {
            let assign = unsafe { &*assign };
            for i in 0..assign.vars.size {
                let var = unsafe { *assign.vars.data.add(i) };
                let expr_local = unsafe { ast_node_as::<AstExprLocal>(var as *mut AstNode) };
                if expr_local.is_null()
                    || !exported_locals.contains(&unsafe { (*expr_local).local })
                {
                    continue;
                }

                let local = unsafe { (*expr_local).local };
                let key = key_of(unsafe { (*local).name });

                if assign.vars.size != assign.values.size || i >= assign.values.size {
                    props.insert(
                        key.clone(),
                        prop_from_ty(lookup_exported_binding_type(local)),
                    );
                } else {
                    let value = unsafe { *assign.values.data.add(i) };
                    props.insert(key.clone(), Property::readonly(lookup_expr_type(value)));
                }

                props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
            }
        } else if !func_stat.is_null() {
            let func_stat = unsafe { &*func_stat };
            let expr_local = unsafe { ast_node_as::<AstExprLocal>(func_stat.name as *mut AstNode) };
            if !expr_local.is_null() && exported_locals.contains(&unsafe { (*expr_local).local }) {
                let local = unsafe { (*expr_local).local };
                let key = key_of(unsafe { (*local).name });
                props.insert(
                    key.clone(),
                    Property::readonly(lookup_expr_type(func_stat.func as *mut AstExpr)),
                );
                props.get_mut(&key).unwrap().location = Some(unsafe { (*local).location });
            }
        } else if FFlag::DebugLuauUserDefinedClasses.get() {
            let class_stat = unsafe { ast_node_as::<AstStatClass>(node) };
            if !class_stat.is_null() {
                let class_stat = unsafe { &*class_stat };
                if !class_stat.exported {
                    continue;
                }

                let key = key_of(unsafe { (*class_stat.name).name });
                props.insert(
                    key.clone(),
                    Property::readonly(lookup_exported_binding_type(class_stat.name)),
                );
                props.get_mut(&key).unwrap().location =
                    Some(unsafe { (*class_stat.name).location });
            }
        }
    }

    if props.is_empty() {
        return;
    }

    let level = unsafe { (*module_scope_ptr).level };
    let exports = module_ref.internal_types.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            level,
            TableState::Sealed,
        ),
    );
    let exports_pack = module_ref.internal_types.add_type_pack_t(TypePack {
        head: vec![exports],
        tail: None,
    });
    unsafe { (*module_scope_ptr).return_type = exports_pack };
}
