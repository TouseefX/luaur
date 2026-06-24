//! Ported from `tests/TypeInfer.loops.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.loops.test.cpp:1061:type_infer_loops_dcr_iteration_fragmented_keys`
//! Source: `tests/TypeInfer.loops.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.loops.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.loops.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_infer_loops_dcr_iteration_fragmented_keys

#[cfg(test)]
#[test]
fn type_infer_loops_dcr_iteration_fragmented_keys() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function isIndexKey(k, contiguousLength)
            return true
        end

        local function getTableLength(tbl)
            local length = 1
            local value = rawget(tbl, length)
            while value ~= nil do
                length += 1
                value = rawget(tbl, length)
            end
            return length - 1
        end

        local function rawpairs(t)
            return next, t, nil
        end

        local function getFragmentedKeys(tbl)
            local keys = {}
            local keysLength = 0
            local tableLength = getTableLength(tbl)
            for key, _ in rawpairs(tbl) do
                if not isIndexKey(key, tableLength) then
                    keysLength = keysLength + 1
                    keys[keysLength] = key
                end
            end
            return keys, keysLength, tableLength
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
