//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:118:type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class

#[cfg(test)]
#[test]
fn type_infer_extern_types_we_can_infer_that_a_parameter_must_be_a_particular_class() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function makeClone(o)
            return BaseClass.Clone(o)
        end

        local a = makeClone(ChildClass.New())
    "#,
        ),
        None,
    );

    assert_eq!(
        "BaseClass",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("a")))
    );
}
