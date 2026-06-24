//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_luau_ice_is_not_special_without_the_flag() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, false);
    let mut fixture = Fixture::fixture_bool(false);

    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: _luau_ice = 55
    "#,
        ),
        None,
    );
}
