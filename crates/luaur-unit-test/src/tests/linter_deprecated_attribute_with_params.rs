//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1873:linter_deprecated_attribute_with_params`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function checkDeprecatedWarning (tests/Linter.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item linter_deprecated_attribute_with_params

#[cfg(test)]
#[test]
fn linter_deprecated_attribute_with_params() {
    use crate::functions::check_deprecated_warning::check_deprecated_warning;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated, use 'prodfun' instead. Too old.",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ use = "prodfun", reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated, use 'prodfun' instead. Too old.",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ use = "prodfun" }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated, use 'prodfun' instead",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ use = "prodfun" }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated, use 'prodfun' instead",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ reason = "Too old." }]
local function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated. Too old.",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@[deprecated{ reason = "Too old." }]
function testfun(x)
    return x + 1
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 7),
            "Function 'testfun' is deprecated. Too old.",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
Account = { balance=0 }

@[deprecated{use = 'credit', reason = 'It sounds cool'}]
function Account:deposit(v)
    self.balance = self.balance + v
end

Account:deposit(200.00)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(8, 0),
            Position::new(8, 15),
            "Member 'Account.deposit' is deprecated, use 'credit' instead. It sounds cool",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
Account = { balance=0 }

function getAccount()
    return Account
end

@[deprecated{use = 'credit', reason = 'It sounds cool'}]
function Account:deposit (v)
    self.balance = self.balance + v
end

(getAccount()):deposit(200.00)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(12, 0),
            Position::new(12, 22),
            "Member 'deposit' is deprecated, use 'credit' instead. It sounds cool",
        );
    }

    {
        fixture.load_definition(
            &String::from(
                r#"
@[deprecated{use = 'foo', reason = 'Do better.'}] declare function bar(x: number): string
"#,
            ),
            false,
        );

        let result = fixture.lint(
            &String::from(
                r#"
bar(2)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(1, 0),
            Position::new(1, 3),
            "Function 'bar' is deprecated, use 'foo' instead. Do better.",
        );
    }

    {
        fixture.load_definition(
            &String::from(
                r#"
declare Hooty : {
    tooty : @[deprecated{use = 'foo', reason = 'bar'}] @checked (number) -> number
}
"#,
            ),
            false,
        );

        let result = fixture.lint(
            &String::from(
                r#"
print(Hooty:tooty(2.0))
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(1, 6),
            Position::new(1, 17),
            "Member 'Hooty.tooty' is deprecated, use 'foo' instead. bar",
        );
    }

    {
        fixture.load_definition(
            &String::from(
                r#"
declare class Foo
   @[deprecated{use = 'foo', reason = 'baz'}]
   function bar(self, value: number) : number
end

declare Foo: {
   new: () -> Foo
}
"#,
            ),
            false,
        );

        let result = fixture.lint(
            &String::from(
                r#"
local foo = Foo.new()
print(foo:bar(2.0))
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(2, 6),
            Position::new(2, 13),
            "Member 'bar' is deprecated, use 'foo' instead. baz",
        );
    }
}
