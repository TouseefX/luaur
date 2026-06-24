//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2047:type_infer_functions_luau_subtyping_is_np_hard`
//! Source: `tests/TypeInfer.functions.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_functions_luau_subtyping_is_np_hard() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict

-- An example of coding up graph coloring in the Luau type system.
-- This codes a three-node, two color problem.
-- A three-node triangle is uncolorable,
-- but a three-node line is colorable.

type Red = "red"
type Blue = "blue"
type Color = Red | Blue
type Coloring = (Color) -> (Color) -> (Color) -> boolean
type Uncolorable = (Color) -> (Color) -> (Color) -> false

type Line = Coloring
  & ((Red) -> (Red) -> (Color) -> false)
  & ((Blue) -> (Blue) -> (Color) -> false)
  & ((Color) -> (Red) -> (Red) -> false)
  & ((Color) -> (Blue) -> (Blue) -> false)

type Triangle = Line
  & ((Red) -> (Color) -> (Red) -> false)
  & ((Blue) -> (Color) -> (Blue) -> false)

local x : Triangle
local y : Line
local z : Uncolorable
z = x -- OK, so the triangle is uncolorable
z = y -- Not OK, so the line is colorable
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t'(\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> false'\nbut got\n\t'((\"blue\" | \"red\") -> (\"blue\" | \"red\") -> (\"blue\" | \"red\") -> boolean) & ((\"blue\" | \"red\") -> (\"blue\") -> (\"blue\") -> false) & ((\"blue\" | \"red\") -> (\"red\") -> (\"red\") -> false) & ((\"blue\") -> (\"blue\") -> (\"blue\" | \"red\") -> false) & ((\"red\") -> (\"red\") -> (\"blue\" | \"red\") -> false)'; none of the intersection parts are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
