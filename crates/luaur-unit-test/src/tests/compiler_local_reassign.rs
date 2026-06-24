#[cfg(test)]
#[test]
fn compiler_local_reassign() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    // locals can be re-assigned and the register gets reused
    let actual =
        compile_function_0("local function test(a, b)\n    local c = a\n    return c + b\nend\n");
    let expected = "\nADD R2 R0 R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // this works if the expression is using type casts or grouping
    let actual = compile_function_0(
        "local function test(a, b)\n    local c = (a :: number)\n    return c + b\nend\n",
    );
    let expected = "\nADD R2 R0 R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // the optimization requires that neither local is mutated
    let actual = compile_function_0(
        "local function test(a, b)\n    local c = a\n    c += 0\n    local d = b\n    b += 0\n    return c + d\nend\n",
    );
    let expected = "\nMOVE R2 R0\nADDK R2 R2 K0 [0]\nMOVE R3 R1\nADDK R1 R1 K0 [0]\nADD R4 R2 R3\nRETURN R4 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // sanity check for two values
    let actual = compile_function_0(
        "local function test(a, b)\n    local c = a\n    local d = b\n    return c + d\nend\n",
    );
    let expected = "\nADD R2 R0 R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // note: we currently only support this for single assignments
    let actual = compile_function_0(
        "local function test(a, b)\n    local c, d = a, b\n    return c + d\nend\n",
    );
    let expected = "\nMOVE R2 R0\nMOVE R3 R1\nADD R4 R2 R3\nRETURN R4 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // of course, captures capture the original register as well (by value since it's immutable)
    let actual = compile_function(
        "local function test(a, b)\n    local c = a\n    local d = b\n    return function() return c + d end\nend\n",
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R2 P0\nCAPTURE VAL R0\nCAPTURE VAL R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
