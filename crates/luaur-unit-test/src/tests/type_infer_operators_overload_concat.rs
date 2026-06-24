//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1676:type_infer_operators_overload_concat`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function writestring (VM/src/lbaselib.cpp)
//!   - translates_to -> rust_item type_infer_operators_overload_concat

#[cfg(test)]
#[test]
fn type_infer_operators_overload_concat() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauConcatDoesntAlwaysReturnString, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type classData = {
            b:buffer;
            len:number;
        }
        local metatable = {
            __concat = function(self:cls,str:string):cls
                buffer.writestring(self.b,self.len,str)
                self.len+=#str
                return self
            end;
        }

        export type cls = typeof(setmetatable({}::classData, metatable))

        --returns a long string
        local new = function():cls
            return setmetatable({
                b = buffer.create(100_000::number);
                len = 0;
            }::classData,metatable)::cls
        end
        local class = new()

        class ..= "Hello"
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
