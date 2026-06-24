//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:273:type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table

#[cfg(test)]
#[test]
fn type_infer_oop_inferred_methods_of_free_tables_have_the_same_level_as_the_enclosing_table() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function Base64FileReader(data)
            local reader = {}
            local index: number = 0

            function reader:PeekByte()
                return data:byte(index)
            end

            function reader:Byte()
                return data:byte(index - 1)
            end

            return reader
        end

        Base64FileReader()

        function ReadMidiEvents(data)

            local reader = Base64FileReader(data)

            while reader:HasMore() do
                (reader:Byte() % 128)
            end
        end
    "#,
        ),
        None,
    );
}
