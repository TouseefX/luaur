//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:341:type_infer_extern_types_table_properties_are_invariant`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_table_properties_are_invariant

#[cfg(test)]
#[test]
fn type_infer_extern_types_table_properties_are_invariant() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(a: {foo: BaseClass})
            a.foo = AnotherChild.New()
        end

        local t: {foo: ChildClass}
        f(t) -- line 6.  Breaks soundness.

        function g(t: {foo: ChildClass})
        end

        local t2: {foo: BaseClass} = {foo=BaseClass.New()}
        t2.foo = AnotherChild.New()
        g(t2) -- line 13.  Breaks soundness
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(6, result.errors[0].location.begin.line);
    assert_eq!(13, result.errors[1].location.begin.line);
}
