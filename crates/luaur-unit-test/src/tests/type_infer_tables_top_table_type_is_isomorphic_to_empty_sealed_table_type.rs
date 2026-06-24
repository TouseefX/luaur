//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4104:type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type

#[cfg(test)]
#[test]
fn type_infer_tables_top_table_type_is_isomorphic_to_empty_sealed_table_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r##"
        local None = newproxy(true)
        local mt = getmetatable(None)
        mt.__tostring = function()
            return "Object.None"
        end

        function assign(...)
            for index = 1, select("#", ...) do
                local rest = select(index, ...)

                if rest ~= nil and typeof(rest) == "table" then
                    for key, value in pairs(rest) do
                    end
                end
            end
        end
    "##,
        ),
        None,
    );
}
