#[cfg(test)]
#[test]
fn compiler_loop_continue_ignores_implicit_constant_after_inline() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local function inline(f)
    repeat
        continue
    until f
end

local function test(...)
    inline(true)
end

test()
"#,
        1,
        2,
        0,
    );
    let expected = "\nRETURN R0 0\nRETURN R0 0\n";

    assert_eq!(format!("\n{}", actual), expected);
}
