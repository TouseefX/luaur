//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1624:linter_deprecated_attribute`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - calls -> function checkDeprecatedWarning (tests/Linter.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item linter_deprecated_attribute

#[cfg(test)]
#[test]
fn linter_deprecated_attribute() {
    use crate::functions::check_deprecated_warning::check_deprecated_warning;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
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
            "Function 'testfun' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
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
            "Function 'testfun' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
local function testfun(x:number):number
    return x + 1
end

if math.random(2) == 2 then
    testfun(1)
end
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(7, 4),
            Position::new(7, 11),
            "Function 'testfun' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
local function testfun(x:number)
    return x + 1
end

g(testfun)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 2),
            Position::new(6, 9),
            "Function 'testfun' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
local function testfun(x):number
    if x == 1 then
        return x
    else
        return 1 + testfun(x - 1)
    end
end

testfun(1)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(10, 0),
            Position::new(10, 7),
            "Function 'testfun' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
function flipFlop()
    local state = false

    @deprecated
    local function invert()
        state = !state
        return state
    end

    return invert
end

f = flipFlop()
assert(f() == true)
"#,
            ),
            None,
        );

        assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(10, 11),
            Position::new(10, 17),
            "Function 'invert' is deprecated",
        );
        check_deprecated_warning(
            &result.warnings[1],
            Position::new(14, 7),
            Position::new(14, 8),
            "Function 'f' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
function flipFlop()
    local state = false

    local function invert()
        state = !state
        return state
    end

    return invert
end

f = flipFlop()
assert(f() == true)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(13, 4),
            Position::new(13, 12),
            "Function 'flipFlop' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
local function doTheThing()
    print("doing")
end

doTheThing()

local function shadow()
    local function doTheThing()
        print("doing!")
    end

    doTheThing()
end

shadow()
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 0),
            Position::new(6, 10),
            "Function 'doTheThing' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
function fibonacci(n)
    if n == 0 then
        return 0
    elseif n == 1 then
        return 1
    else
        return fibonacci(n - 1) + fibonacci(n - 2)
    end
end

fibonacci(5)
"#,
            ),
            None,
        );

        assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(12, 0),
            Position::new(12, 9),
            "Function 'fibonacci' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
@deprecated
function odd(x)
    if x == 0 then
        return false
    else
        return even(x - 1)
    end
end

@deprecated
function even(x)
    if x == 0 then
        return true
    else
        return odd(x - 1)
    end
end

assert(odd(1) == true)
assert(even(0) == true)
"#,
            ),
            None,
        );

        assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
        check_deprecated_warning(
            &result.warnings[0],
            Position::new(6, 15),
            Position::new(6, 19),
            "Function 'even' is deprecated",
        );
        check_deprecated_warning(
            &result.warnings[1],
            Position::new(15, 15),
            Position::new(15, 18),
            "Function 'odd' is deprecated",
        );
        check_deprecated_warning(
            &result.warnings[2],
            Position::new(19, 7),
            Position::new(19, 10),
            "Function 'odd' is deprecated",
        );
        check_deprecated_warning(
            &result.warnings[3],
            Position::new(20, 7),
            Position::new(20, 11),
            "Function 'even' is deprecated",
        );
    }

    {
        let result = fixture.lint(
            &String::from(
                r#"
Account = { balance=0 }

@deprecated
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
            "Member 'Account.deposit' is deprecated",
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

@deprecated
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
            "Member 'deposit' is deprecated",
        );
    }
}
