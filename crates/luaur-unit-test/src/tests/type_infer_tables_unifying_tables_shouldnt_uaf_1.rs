//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2836:type_infer_tables_unifying_tables_shouldnt_uaf_1`
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
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - translates_to -> rust_item type_infer_tables_unifying_tables_shouldnt_uaf_1

#[cfg(test)]
#[test]
fn type_infer_tables_unifying_tables_shouldnt_uaf_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
-- This example produced a UAF at one point, caused by pointers to table types becoming
-- invalidated by child unifiers. (Calling log.concat can cause pointers to become invalid.)
type _Entry = {
    a: number,

    middle: (self: _Entry) -> (),

    z: number
}

export type AnyEntry = _Entry

local Entry = {}
Entry.__index = Entry

function Entry:dispose()
    self:middle()
    forgetChildren(self) -- unify free with sealed AnyEntry
end

function forgetChildren(parent: AnyEntry)
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
