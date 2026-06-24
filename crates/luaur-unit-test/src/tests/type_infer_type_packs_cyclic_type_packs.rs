//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:996:type_infer_type_packs_cyclic_type_packs`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_type_packs_cyclic_type_packs

#[cfg(test)]
#[test]
fn type_infer_type_packs_cyclic_type_packs() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!nonstrict
_ += _(_,...)
repeat
_ += _(...)
until ... + _
"#,
        ),
        None,
    );

    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!nonstrict
_ += _(_(...,...),_(...))
repeat
until _
"#,
        ),
        None,
    );
}
