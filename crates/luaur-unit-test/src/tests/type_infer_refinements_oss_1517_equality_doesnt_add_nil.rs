//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2992:type_infer_refinements_oss_1517_equality_doesnt_add_nil`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_oss_1517_equality_doesnt_add_nil

#[cfg(test)]
#[test]
fn type_infer_refinements_oss_1517_equality_doesnt_add_nil() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyType = {
            data: any
        }

        local function createMyType(): MyType
            local obj = { data = {} }
            return obj
        end

        local function testTypeInference()
            local a: MyType = createMyType()
            local b: MyType = createMyType()

            if a == b then
                local c: MyType = b
                local value = b.data
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
