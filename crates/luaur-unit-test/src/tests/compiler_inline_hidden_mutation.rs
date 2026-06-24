#[cfg(test)]
#[test]
fn compiler_inline_hidden_mutation() {
    use crate::functions::compile_function::compile_function;

    // when the argument is assigned inside the function, we can't reuse the local
    let actual = compile_function(
        r#"
local function foo(a)
    a = 42
    return a
end

local x = ...
local y = foo(x :: number)
return y
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 1\nMOVE R3 R1\nLOADN R3 42\nMOVE R2 R3\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // and neither can we do that when it's assigned outside the function
    let actual = compile_function(
        r#"
local function foo(a)
    mutator()
    return a
end

local x = ...
mutator = function() x = 42 end

local y = foo(x :: number)
return y
"#,
        2,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 1\nNEWCLOSURE R2 P1\nCAPTURE REF R1\nSETGLOBAL R2 K1 ['mutator']\nMOVE R3 R1\nGETGLOBAL R4 K1 ['mutator']\nCALL R4 0 0\nMOVE R2 R3\nCLOSEUPVALS R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
