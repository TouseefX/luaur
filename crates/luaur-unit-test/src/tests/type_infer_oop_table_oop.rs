//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:304:type_infer_oop_table_oop`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - translates_to -> rust_item type_infer_oop_table_oop

#[cfg(test)]
#[test]
fn type_infer_oop_table_oop() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
   --!strict
local Class = {}
Class.__index = Class

type Class = typeof(setmetatable({} :: { x: number }, Class))

function Class.new(x: number): Class
    return setmetatable({x = x}, Class)
end

function Class.getx(self: Class)
    return self.x
end

function test()
    local c = Class.new(42)
    local n = c:getx()
    local nn = c.x

    print(string.format("%d %d", n, nn))
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
