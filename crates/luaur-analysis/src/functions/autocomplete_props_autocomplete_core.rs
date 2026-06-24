use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::enums::parentheses_recommendation::ParenthesesRecommendation;
use crate::enums::prop_index_type::PropIndexType;
use crate::enums::type_correct_kind::TypeCorrectKind;
use crate::functions::check_type_correct_kind::check_type_correct_kind;
use crate::functions::check_type_match::check_type_match;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_paren_recommendation::get_paren_recommendation;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::module::Module;
use crate::records::primitive_type::PrimitiveType;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_arena::TypeArena;
use crate::records::union_type::UnionType;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::sync::Arc;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;

// C++ `kParseNameError` from ParseResult.h.
const K_PARSE_NAME_ERROR: &str = "%error-id%";

/// C++ `static void autocompleteProps(...)` (AutocompleteCore.cpp:267-545), the
/// full nine-parameter recursive overload.
#[allow(clippy::too_many_arguments)]
pub fn autocomplete_props(
    module: &Module,
    type_arena: *mut TypeArena,
    builtin_types: &BuiltinTypes,
    root_ty: TypeId,
    ty: TypeId,
    index_type: PropIndexType,
    nodes: &alloc::vec::Vec<*mut AstNode>,
    result: &mut AutocompleteEntryMap,
    seen: &mut std::collections::HashSet<TypeId>,
    containing_extern_type: Option<*const ExternType>,
) {
    let root_ty = unsafe { follow_type_id(root_ty) };
    let ty = unsafe { follow_type_id(ty) };

    if seen.contains(&ty) {
        return;
    }
    seen.insert(ty);

    let module_scope = module.get_module_scope();
    let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;
    let builtin_types_ptr = builtin_types as *const BuiltinTypes as *mut BuiltinTypes;

    // `isWrongIndexer` lambda from C++.
    let is_wrong_indexer = |type_id: TypeId| -> bool {
        if index_type == PropIndexType::Key {
            return false;
        }

        let called_with_self = index_type == PropIndexType::Colon;

        // `isCompatibleCall` nested lambda.
        let is_compatible_call = |ftv: &FunctionType| -> bool {
            // Strong match with definition is a success
            if called_with_self == ftv.has_self {
                return true;
            }

            // Calls on extern types require strict match between how function is
            // declared and how it's called
            if !unsafe { get_type_id::<ExternType>(root_ty) }.is_null() {
                return false;
            }

            // When called with ':', but declared without 'self', it is invalid if
            // a function has incompatible first argument or no arguments at all.
            // When called with '.', but declared with 'self', it is considered
            // invalid if first argument is compatible.
            if let Some(first_arg_ty) = first(ftv.arg_types, true) {
                if check_type_match(
                    module,
                    root_ty,
                    first_arg_ty,
                    module_scope_ptr,
                    type_arena,
                    builtin_types_ptr,
                ) {
                    return called_with_self;
                }
            }

            !called_with_self
        };

        let ftv = unsafe { get_type_id::<FunctionType>(type_id) };
        if !ftv.is_null() {
            return !is_compatible_call(unsafe { &*ftv });
        }

        // For intersections, any part that is successful makes the whole call successful
        let itv = unsafe { get_type_id::<IntersectionType>(type_id) };
        if !itv.is_null() {
            for &sub_type in unsafe { &(*itv).parts } {
                let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(sub_type)) };
                if !ftv.is_null() && is_compatible_call(unsafe { &*ftv }) {
                    return false;
                }
            }
        }

        called_with_self
    };

    // `maybeFillSingletonProp` lambda from C++.
    let maybe_fill_singleton_prop = |result: &mut AutocompleteEntryMap, type_id: TypeId| {
        let singleton_ty = unsafe { get_type_id::<SingletonType>(type_id) };
        if !singleton_ty.is_null() {
            let string_singleton =
                get_singleton_type::<StringSingleton>(singleton_ty as *const SingletonType);
            if !string_singleton.is_null() {
                let string_singleton = unsafe { &*string_singleton };

                let type_correct = if index_type == PropIndexType::Key {
                    TypeCorrectKind::Correct
                } else {
                    check_type_correct_kind(
                        module,
                        type_arena,
                        builtin_types,
                        *nodes.last().unwrap(),
                        Position::default(),
                        type_id,
                    )
                };

                let parens = if index_type == PropIndexType::Key {
                    ParenthesesRecommendation::None
                } else {
                    get_paren_recommendation(ty, nodes, type_correct)
                };

                result.insert(
                    string_singleton.value.clone(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::String,
                        r#type: Some(type_id),
                        deprecated: false,
                        wrong_index_type: is_wrong_indexer(type_id),
                        type_correct,
                        containing_extern_type,
                        prop: None,
                        documentation_symbol: None,
                        tags: Default::default(),
                        parens,
                        insert_text: None,
                        indexed_with_self: index_type == PropIndexType::Colon,
                    },
                );
            }
        }
    };

    // `fillProps` lambda from C++.
    let fill_props = |result: &mut AutocompleteEntryMap, props: &Props| {
        for (name, prop) in props {
            // We are walking up the class hierarchy, so if we encounter a property
            // that we have already populated, it takes precedence over the
            // property we found just now.
            if !result.contains_key(name) && name != K_PARSE_NAME_ERROR {
                let type_id: TypeId = if let Some(ty) = prop.read_ty {
                    unsafe { follow_type_id(ty) }
                } else {
                    continue;
                };

                let type_correct = if index_type == PropIndexType::Key {
                    TypeCorrectKind::Correct
                } else {
                    check_type_correct_kind(
                        module,
                        type_arena,
                        builtin_types,
                        *nodes.last().unwrap(),
                        Position::default(),
                        type_id,
                    )
                };

                let parens = if index_type == PropIndexType::Key {
                    ParenthesesRecommendation::None
                } else {
                    get_paren_recommendation(type_id, nodes, type_correct)
                };

                result.insert(
                    name.clone(),
                    AutocompleteEntry {
                        kind: AutocompleteEntryKind::Property,
                        r#type: Some(type_id),
                        deprecated: prop.deprecated,
                        wrong_index_type: is_wrong_indexer(type_id),
                        type_correct,
                        containing_extern_type,
                        prop: Some(
                            prop as *const Property
                                as *const crate::records::property_type_path::Property,
                        ),
                        documentation_symbol: prop.documentation_symbol.clone(),
                        tags: Default::default(),
                        parens,
                        insert_text: None,
                        indexed_with_self: index_type == PropIndexType::Colon,
                    },
                );
            }
        }
    };

    // `fillMetatableProps` lambda from C++.
    let fill_metatable_props = |result: &mut AutocompleteEntryMap,
                                seen: &mut std::collections::HashSet<TypeId>,
                                mtable: *const TableType| {
        let mtable = unsafe { &*mtable };
        if let Some(index_prop) = mtable.props.get("__index") {
            let followed = match index_prop.read_ty {
                Some(t) => t,
                None => return,
            };
            let followed = unsafe { follow_type_id(followed) };

            if !unsafe { get_type_id::<TableType>(followed) }.is_null()
                || !unsafe { get_type_id::<MetatableType>(followed) }.is_null()
            {
                autocomplete_props(
                    module,
                    type_arena,
                    builtin_types,
                    root_ty,
                    followed,
                    index_type,
                    nodes,
                    result,
                    seen,
                    None,
                );
            } else {
                let index_function = unsafe { get_type_id::<FunctionType>(followed) };
                if !index_function.is_null() {
                    let index_function_result = first(unsafe { (*index_function).ret_types }, true);
                    if let Some(index_function_result) = index_function_result {
                        autocomplete_props(
                            module,
                            type_arena,
                            builtin_types,
                            root_ty,
                            index_function_result,
                            index_type,
                            nodes,
                            result,
                            seen,
                            None,
                        );
                    }
                }
            }
        }
    };

    let cls = unsafe { get_type_id::<ExternType>(ty) };
    let tbl = unsafe { get_type_id::<TableType>(ty) };
    let mt = unsafe { get_type_id::<MetatableType>(ty) };
    let i = unsafe { get_type_id::<IntersectionType>(ty) };
    let u = unsafe { get_type_id::<UnionType>(ty) };
    let pt = unsafe { get_type_id::<PrimitiveType>(ty) };

    if !cls.is_null() {
        let cls = unsafe { &*cls };
        let containing_extern_type = containing_extern_type.or(Some(cls as *const ExternType));
        fill_props(result, &cls.props);
        if let Some(parent) = cls.parent {
            autocomplete_props(
                module,
                type_arena,
                builtin_types,
                root_ty,
                parent,
                index_type,
                nodes,
                result,
                seen,
                containing_extern_type,
            );
        }
    } else if !tbl.is_null() {
        let tbl = unsafe { &*tbl };
        fill_props(result, &tbl.props);
        if tbl.indexer.is_some() && index_type == PropIndexType::Point {
            let indexer_ty = unsafe { follow_type_id(tbl.indexer.as_ref().unwrap().index_type) };
            let utv = unsafe { get_type_id::<UnionType>(indexer_ty) };
            if !utv.is_null() {
                for &option in unsafe { &(*utv).options } {
                    maybe_fill_singleton_prop(result, option);
                }
            } else {
                maybe_fill_singleton_prop(result, indexer_ty);
            }
        }
    } else if !mt.is_null() {
        let mt = unsafe { &*mt };
        autocomplete_props(
            module,
            type_arena,
            builtin_types,
            root_ty,
            mt.table,
            index_type,
            nodes,
            result,
            seen,
            None,
        );

        let mtable = unsafe { get_type_id::<TableType>(follow_type_id(mt.metatable)) };
        if !mtable.is_null() {
            fill_metatable_props(result, seen, mtable);
        }
    } else if !i.is_null() {
        // Complete all properties in every variant
        for &ty in unsafe { &(*i).parts } {
            let mut inner: AutocompleteEntryMap = Default::default();
            let mut inner_seen: std::collections::HashSet<TypeId> = seen.clone();

            autocomplete_props(
                module,
                type_arena,
                builtin_types,
                root_ty,
                ty,
                index_type,
                nodes,
                &mut inner,
                &mut inner_seen,
                None,
            );

            for (k, v) in inner {
                result.entry(k).or_insert(v);
            }
        }
    } else if !u.is_null() {
        // Complete all properties common to all variants
        let options = unsafe { &(*u).options };
        let mut idx = 0usize;

        while idx < options.len() {
            if is_nil(options[idx]) {
                idx += 1;
            } else {
                break;
            }
        }

        if idx == options.len() {
            return;
        }

        autocomplete_props(
            module,
            type_arena,
            builtin_types,
            root_ty,
            options[idx],
            index_type,
            nodes,
            result,
            seen,
            None,
        );

        idx += 1;

        while idx < options.len() {
            let mut inner: AutocompleteEntryMap = Default::default();
            let mut inner_seen: std::collections::HashSet<TypeId> = Default::default();

            // If we don't do this, and we have the misfortune of receiving a
            // recursive union like:
            //
            //  t1 where t1 = t1 | ExternType
            //
            // Then we are on a one way journey to a stack overflow.
            for &ty in seen.iter() {
                if !unsafe { get_type_id::<UnionType>(ty) }.is_null()
                    || !unsafe { get_type_id::<IntersectionType>(ty) }.is_null()
                {
                    inner_seen.insert(ty);
                }
            }

            if is_nil(options[idx]) {
                idx += 1;
                continue;
            }

            autocomplete_props(
                module,
                type_arena,
                builtin_types,
                root_ty,
                options[idx],
                index_type,
                nodes,
                &mut inner,
                &mut inner_seen,
                None,
            );

            let mut to_remove: alloc::collections::BTreeSet<String> = Default::default();

            for (k, _v) in result.iter() {
                if !inner.contains_key(k) {
                    to_remove.insert(k.clone());
                }
            }

            for k in to_remove {
                result.remove(&k);
            }

            idx += 1;
        }
    } else if !pt.is_null() {
        let pt = unsafe { &*pt };
        if let Some(metatable) = pt.metatable {
            let mtable = unsafe { get_type_id::<TableType>(metatable) };
            if !mtable.is_null() {
                fill_metatable_props(result, seen, mtable);
            }
        }
    } else {
        let singleton = unsafe { get_type_id::<SingletonType>(ty) };
        let string_singleton =
            get_singleton_type::<StringSingleton>(singleton as *const SingletonType);
        if !string_singleton.is_null() {
            autocomplete_props(
                module,
                type_arena,
                builtin_types,
                root_ty,
                builtin_types.stringType,
                index_type,
                nodes,
                result,
                seen,
                None,
            );
        }
    }
}
