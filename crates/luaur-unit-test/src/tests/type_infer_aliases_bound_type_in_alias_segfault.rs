//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_bound_type_in_alias_segfault() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        type Map<T, V> = {[K]: V}
        function foo:bar(): Config<any, any> end
        type Config<TSource, TContext> = Map<TSource, TContext> & { fields: FieldConfigMap<any, any>}
        export type FieldConfig<TSource, TContext, TArgs> = {[string]: any}
        export type FieldConfigMap<TSource, TContext> = Map<string, FieldConfig<TSource, TContext>>
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}
