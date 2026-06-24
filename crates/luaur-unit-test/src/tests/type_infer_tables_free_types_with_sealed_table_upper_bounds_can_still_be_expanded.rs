#[cfg(test)]
#[test]
fn type_infer_tables_free_types_with_sealed_table_upper_bounds_can_still_be_expanded() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function bar(a: {x: number}) end

        function foo(a)
            bar(a)

            -- Here, a : A where A = never <: A <: {x: number}
            -- The upper bound of A is a sealed table, but we nevertheless want to extend it.
            a.nope()
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "({ read nope: () -> (...unknown) } & { x: number }) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
