//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:203:type_infer_oop_object_constructor_can_refer_to_method_of_self`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_oop_object_constructor_can_refer_to_method_of_self

#[cfg(test)]
#[test]
fn type_infer_oop_object_constructor_can_refer_to_method_of_self() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        type Foo = {
            fooConn: () -> () | nil
        }

        local Foo = {}
        Foo.__index = Foo

        function Foo.new()
            local self: Foo = {
                fooConn = nil,
            }
            setmetatable(self, Foo)

            self.fooConn = function()
                self:method() -- Key 'method' not found in table self
            end

            return self
        end

        function Foo:method()
            print("foo")
        end

        local foo = Foo.new()

        -- TODO This is the best our current refinement support can offer :(
        local bar = foo.fooConn
        if bar then bar() end

        -- foo.fooConn()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
