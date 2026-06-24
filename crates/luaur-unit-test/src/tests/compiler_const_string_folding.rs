use alloc::string::String;

use crate::functions::compile_function::compile_function;
use crate::functions::format::format;
use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

#[cfg(test)]
#[test]
fn compiler_const_string_folding() {
    let _emit_call_fb: ScopedFastFlag =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);
    let _luau_compile_string_interp_temp_reg: ScopedFastFlag =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauCompileStringInterpTargetTop, true);

    assert_eq!(
        "\n".to_string() + &compile_function(r#"return "" .. """#, 0, 2, 0),
        "\n\
LOADK R0 K0 ['']\n\
RETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function(r#"return "a" .. """#, 0, 2, 0),
        "\n\
LOADK R0 K0 ['a']\n\
RETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function(r#"return "" .. "a""#, 0, 2, 0),
        "\n\
LOADK R0 K0 ['a']\n\
RETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function(
                r#"local hello = "hello"; local world = "world"; return hello .. " " .. world"#,
                0,
                2,
                0
            ),
        "\n\
LOADK R0 K0 ['hello world']\n\
RETURN R0 1\n"
    );

    assert_eq!(
        "\n\
"
        .to_string()
            + &compile_function(
                r#"
local a1 = "0123456789012345678901234567890123456789"
local a2 = a1 .. a1 .. a1 .. a1 .. a1 .. a1 .. a1 .. a1 .. a1 .. a1
local a3 = a2 .. a2 .. a2 .. a2 .. a2 .. a2 .. a2 .. a2 .. a2 .. a2
local a4 = a3 .. a3 .. a3 .. a3 .. a3 .. a3 .. a3 .. a3 .. a3 .. a3
local a5 = a4 .. a4 .. a4 .. a4 .. a4 .. a4 .. a4 .. a4 .. a4 .. a4
return a5
"#,
                0,
                2,
                0
            ),
        "\n\
LOADK R1 K0 ['01234567890123456789012345678901'...]\n\
LOADK R2 K0 ['01234567890123456789012345678901'...]\n\
LOADK R3 K0 ['01234567890123456789012345678901'...]\n\
LOADK R4 K0 ['01234567890123456789012345678901'...]\n\
LOADK R5 K0 ['01234567890123456789012345678901'...]\n\
LOADK R6 K0 ['01234567890123456789012345678901'...]\n\
LOADK R7 K0 ['01234567890123456789012345678901'...]\n\
LOADK R8 K0 ['01234567890123456789012345678901'...]\n\
LOADK R9 K0 ['01234567890123456789012345678901'...]\n\
LOADK R10 K0 ['01234567890123456789012345678901'...]\n\
CONCAT R0 R1 R10\n\
MOVE R2 R0\n\
MOVE R3 R0\n\
MOVE R4 R0\n\
MOVE R5 R0\n\
MOVE R6 R0\n\
MOVE R7 R0\n\
MOVE R8 R0\n\
MOVE R9 R0\n\
MOVE R10 R0\n\
MOVE R11 R0\n\
CONCAT R1 R2 R11\n\
RETURN R1 1\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function(
                r#"
local a1 = "0123456789012345678901234567890123456789"
local a2 = `{a1}{a1}{a1}{a1}{a1}{a1}{a1}{a1}{a1}{a1}`
local a3 = `{a2}{a2}{a2}{a2}{a2}{a2}{a2}{a2}{a2}{a2}`
local a4 = `{a3}{a3}{a3}{a3}{a3}{a3}{a3}{a3}{a3}{a3}`
local a5 = `{a4}{a4}{a4}{a4}{a4}{a4}{a4}{a4}{a4}{a4}`
return a5
"#,
                0,
                2,
                0
            ),
        "\n\
LOADK R0 K0 ['01234567890123456789012345678901'...]\n\
NAMECALL R0 R0 K1 ['format']\n\
CALL R0 1 1\n\
LOADK R1 K2 ['%*%*%*%*%*%*%*%*%*%*']\n\
MOVE R3 R0\n\
MOVE R4 R0\n\
MOVE R5 R0\n\
MOVE R6 R0\n\
MOVE R7 R0\n\
MOVE R8 R0\n\
MOVE R9 R0\n\
MOVE R10 R0\n\
MOVE R11 R0\n\
MOVE R12 R0\n\
NAMECALL R1 R1 K1 ['format']\n\
CALL R1 11 1\n\
RETURN R1 1\n"
    );

    // Silence unused import warnings for format (kept to mirror C++ CHECK_EQ formatting path if used by harness).
    let _ = format as fn(&str, &str, usize, usize) -> String;
}
