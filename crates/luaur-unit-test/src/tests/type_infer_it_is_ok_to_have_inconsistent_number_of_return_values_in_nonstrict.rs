//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1338:type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict

#[cfg(test)]
#[test]
fn type_infer_it_is_ok_to_have_inconsistent_number_of_return_values_in_nonstrict() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        function validate(stats, hits, misses)
            local checked = {}

            for _,l in ipairs(hits) do
                if not (stats[l] and stats[l] > 0) then
                    return false, string.format("expected line %d to be hit", l)
                end
                checked[l] = true
            end

            for _,l in ipairs(misses) do
                if not (stats[l] and stats[l] == 0) then
                    return false, string.format("expected line %d to be missed", l)
                end
                checked[l] = true
            end

            for k,v in pairs(stats) do
                if type(k) == "number" and not checked[k] then
                    return false, string.format("expected line %d to be absent", k)
                end
            end

            return true
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
