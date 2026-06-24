#[cfg(test)]
#[test]
fn compiler_inline_non_argument_const_conditionals() {
    use crate::functions::compile_function::compile_function;

    let actual1 = compile_function(
        r#"local test = false

local function foo(a)
    if test then
        for i = 1,10 do
            print(table.unpack(table.create(100, i)))
        end
    end
    return a + 42
end

local x = foo(1)
return x
"#,
        1,
        2,
        0,
    );
    let expected1 = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 43\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function(
        r#"local test = true

local function foo(a)
    if not test then
        for i = 1,10 do
            print(table.unpack(table.create(100, i)))
        end
    end
    return a + 42
end

local x = foo(1)
return x
"#,
        1,
        2,
        0,
    );
    let expected2 = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 43\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function(
        r#"local test = false

local function foo(a)
    if not test then
        return a + 42
    end

    for i = 1,10 do
        print(table.unpack(table.create(100, i)))
    end
end

local x = foo(1)
return x
"#,
        1,
        2,
        0,
    );
    let expected3 = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 43\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual3), expected3);
}
