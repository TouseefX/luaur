//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3195:type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type`
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
//!   - type_ref -> enum State (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type

#[cfg(test)]
#[test]
fn type_infer_refinements_cli_184413_refinement_of_union_of_read_types_is_read_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type States = "Closed" | "Closing" | "Opening" | "Open"
        export type MyType<A = any> = {
            State: States,
            IsOpen: boolean,
            Open: (self: MyType<A>) -> (),
        }

        local value = {} :: MyType

        function value:Open()
            if self.IsOpen == true then
            elseif self.State == "Closing" or self.State == "Opening" then
                -- Prior, this line errored as we were erroneously refining
                -- `self` with `{ State: "Closing" | "Opening" }` rather
                -- than `{ read State: "Closing" | "Opening" }
                self:Open()
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
