#[cfg(test)]
#[test]
fn compiler_loop_unroll_basic() {
    use crate::functions::compile_function::compile_function;

    // forward loops
    let actual1 = "\n".to_string()
        + &compile_function(
            "local t = {}\nfor i=1,2 do\n    t[i] = i\nend\nreturn t\n",
            0,
            2,
            0,
        );
    let expected1 = "\n\
NEWTABLE R0 0 2
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 2
SETTABLEN R1 R0 2
RETURN R0 1
";
    assert_eq!(actual1, expected1);

    // backward loops
    let actual2 = "\n".to_string()
        + &compile_function(
            "local t = {}\nfor i=2,1,-1 do\n    t[i] = i\nend\nreturn t\n",
            0,
            2,
            0,
        );
    let expected2 = "\n\
NEWTABLE R0 0 0
LOADN R1 2
SETTABLEN R1 R0 2
LOADN R1 1
SETTABLEN R1 R0 1
RETURN R0 1
";
    assert_eq!(actual2, expected2);

    // loops with step that doesn't divide to-from
    let actual3 = "\n".to_string()
        + &compile_function(
            "local t = {}\nfor i=1,4,2 do\n    t[i] = i\nend\nreturn t\n",
            0,
            2,
            0,
        );
    let expected3 = "\n\
NEWTABLE R0 0 0
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 3
SETTABLEN R1 R0 3
RETURN R0 1
";
    assert_eq!(actual3, expected3);

    // empty loops
    let actual4 = "\n".to_string() + &compile_function("for i=2,1 do\nend\n", 0, 2, 0);
    let expected4 = "\n\
RETURN R0 0
";
    assert_eq!(actual4, expected4);
}
