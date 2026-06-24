//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.anyerror.test.cpp:346:type_infer_anyerror_metatable_of_any_can_be_a_table`
//! Source: `tests/TypeInfer.anyerror.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.anyerror.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.anyerror.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_anyerror_metatable_of_any_can_be_a_table

#[cfg(test)]
#[test]
fn type_infer_anyerror_metatable_of_any_can_be_a_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local T: any
T = {}
T.__index = T
function T.new(...)
    local self = {}
    setmetatable(self, T)
    self:construct(...)
    return self
end
function T:construct(index)
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
