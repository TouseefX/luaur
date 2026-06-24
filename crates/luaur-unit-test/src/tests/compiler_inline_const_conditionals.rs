#[cfg(test)]
#[test]
fn compiler_inline_const_conditionals() {
    use crate::functions::compile_function::compile_function;

    assert_eq!(
        format!(
            "\n{}",
            compile_function(
                r#"
local function foo(a)
    if a == 1 then
        return 42
    elseif a == 2 then
        return -1
    else
        for i = 1,10 do
            print(table.unpack(table.create(100, i)))
        end
    end
end

local x = foo(1)
local y = foo(2)
return x, y
"#,
                1,
                2,
                0
            )
        ),
        "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 42\nLOADN R2 -1\nRETURN R1 2\n"
    );

    assert_eq!(
        format!(
            "\n{}",
            compile_function(
                r#"
local function foo(a)
    local s = 0
    for i = 1,5 do
        if a == 1 then
            s += i
        elseif a == 2 then
            s -= i
        else
            print(table.unpack(table.create(100, i)))
        end
    end
    return s
end

local x = foo(1)
local y = foo(2)
return x, y
"#,
                1,
                2,
                0
            )
        ),
        "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R2 0\nADDK R2 R2 K1 [1]\nADDK R2 R2 K2 [2]\nADDK R2 R2 K3 [3]\nADDK R2 R2 K4 [4]\nADDK R2 R2 K5 [5]\nMOVE R1 R2\nLOADN R3 0\nSUBK R3 R3 K1 [1]\nSUBK R3 R3 K2 [2]\nSUBK R3 R3 K3 [3]\nSUBK R3 R3 K4 [4]\nSUBK R3 R3 K5 [5]\nMOVE R2 R3\nRETURN R1 2\n"
    );

    assert_eq!(
        format!(
            "\n{}",
            compile_function(
                r#"
local function foo(a, b, c, d)
    return if a > 10 then a + b else magic({a, b, c}, {d})
end

local x = foo(20, 1, 2, 3, 4, 5)
return x
"#,
                1,
                2,
                0
            )
        ),
        "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 21\nRETURN R1 1\n"
    );

    assert_eq!(
        format!(
            "\n{}",
            compile_function(
                r#"
local function funnyhex(a)
    local z = string.byte('0')
    local set = "0123456789abcdef"
    if a < 10 then return string.sub(set, a+1, a+1)
    elseif a < 100 then return `{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}`
    elseif a < 1000 then return `{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}`
    elseif a < 10000 then return `{string.sub(set, (a/1000)%10+1, (a/1000)%10+1)}{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}`
    elseif a < 100000 then return `{string.sub(set, (a/10000)%10+1, (a/10000)%10+1)}{string.sub(set, (a/1000)%10+1, (a/1000)%10+1)}{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}`
    else return tostring(a) end
end

local a = funnyhex(1)
local b = funnyhex(24)
local c = funnyhex(560)
local d = funnyhex(8943)
local e = funnyhex(46825)
return a, b, c, d, e
"#,
                1,
                2,
                0
            )
        ),
        "\nDUPCLOSURE R0 K0 ['funnyhex']\nLOADK R1 K1 ['1']\nLOADK R2 K2 ['24']\nLOADK R3 K3 ['560']\nLOADK R4 K4 ['8943']\nLOADK R5 K5 ['46825']\nRETURN R1 5\n"
    );

    assert_eq!(
        format!(
            "\n{}",
            compile_function(
                r#"
local function funnyhex(a)
    local z = string.byte('0')
    local set = "0123456789abcdef"
    if a < 10 then return string.sub(set, a+1, a+1) end
    if a < 100 then return `{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}` end
    if a < 1000 then return `{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}` end
    if a < 10000 then return `{string.sub(set, (a/1000)%10+1, (a/1000)%10+1)}{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}` end
    if a < 100000 then return `{string.sub(set, (a/10000)%10+1, (a/10000)%10+1)}{string.sub(set, (a/1000)%10+1, (a/1000)%10+1)}{string.sub(set, (a/100)%10+1, (a/100)%10+1)}{string.sub(set, (a/10)%10+1, (a/10)%10+1)}{string.sub(set, a%10+1, a%10+1)}` end
    return tostring(a)
end

local a = funnyhex(1)
local b = funnyhex(24)
local c = funnyhex(560)
local d = funnyhex(8943)
local e = funnyhex(46825)
return a, b, c, d, e
"#,
                1,
                2,
                0
            )
        ),
        "\nDUPCLOSURE R0 K0 ['funnyhex']\nLOADK R1 K1 ['1']\nLOADK R2 K2 ['24']\nLOADK R3 K3 ['560']\nLOADK R4 K4 ['8943']\nLOADK R5 K5 ['46825']\nRETURN R1 5\n"
    );
}
