//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_indexing_into_a_cyclic_union_doesnt_crash() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::builtin_types::BuiltinTypes;
    use luaur_analysis::records::frontend::Frontend;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_indexer::TableIndexer;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_fun::TypeFun;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::props_type::Props;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = Fixture::fixture_bool(false);
    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let builtins_ptr = fixture.get_builtins() as *mut BuiltinTypes;

    unsafe {
        let frontend = &mut *frontend_ptr;
        let global_scope = frontend.globals.global_scope();
        let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
        let arena = frontend.globals.global_types_mut();

        unfreeze(arena);

        let bad_cyclic_union_ty =
            arena.fresh_type_not_null_builtin_types_scope(&*builtins_ptr, global_scope_ptr);
        let number_type = (*builtins_ptr).numberType;
        let props: Props = Default::default();
        let number_array_ty = arena.add_type(
            TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                &props,
                Some(TableIndexer {
                    index_type: number_type,
                    index_result_type: number_type,
                    is_read_only: false,
                }),
                TypeLevel::default(),
                global_scope_ptr,
                TableState::Sealed,
            ),
        );

        (*as_mutable_type_id(bad_cyclic_union_ty)).ty = TypeVariant::Union(UnionType {
            options: vec![bad_cyclic_union_ty, number_array_ty],
        });

        (*global_scope_ptr).exported_type_bindings.insert(
            String::from("BadCyclicUnion"),
            TypeFun::type_fun_type_id(bad_cyclic_union_ty),
        );

        freeze(arena);
    }

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x: BadCyclicUnion)
            return x[0]
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
