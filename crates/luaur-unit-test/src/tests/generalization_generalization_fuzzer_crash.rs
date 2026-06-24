//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:455:generalization_generalization_fuzzer_crash`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item generalization_generalization_fuzzer_crash

#[cfg(test)]
#[test]
fn generalization_generalization_fuzzer_crash() {
    use crate::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function t0<A>(l0,...):""
        type t0 = any
        do
        _()
        _ = {_=...,}
        _ = {_=rawget({_=_,l0,},_,- _),}
        end
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
