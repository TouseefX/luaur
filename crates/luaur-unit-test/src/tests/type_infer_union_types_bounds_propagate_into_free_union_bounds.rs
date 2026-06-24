//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_bounds_propagate_into_free_union_bounds() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(
        &FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
        true,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function unwrap<T>(a: T?): T
            if a == nil then
                error("Unexpected nil!")
            end
            return a
        end

        local b = unwrap(42)
        local c = unwrap(true)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
}
