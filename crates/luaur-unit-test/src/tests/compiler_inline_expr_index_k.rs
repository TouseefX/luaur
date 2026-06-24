#[cfg(test)]
#[test]
fn compiler_inline_expr_index_k() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local _ = function(l0)
local _ = nil
while _(_)[_] do
end
end
local _ = _(0)[""]
if _ then
do
for l0=0,8 do
end
end
elseif _ then
_ = nil
do
for l0=0,8 do
return true
end
end
end"#,
        1,
        2,
        0,
    );

    let expected = r#"
DUPCLOSURE R0 K0 []
L0: LOADNIL R4
LOADNIL R5
CALL R4 1 1
LOADNIL R5
GETTABLE R3 R4 R5
JUMPIFNOT R3 L1
JUMPBACK L0
L1: LOADNIL R2
GETTABLEKS R1 R2 K1 ['']
JUMPIFNOT R1 L2
RETURN R0 0
L2: JUMPIFNOT R1 L3
LOADNIL R1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
LOADB R2 1
RETURN R2 1
L3: RETURN R0 0
"#;

    assert_eq!(format!("\n{}", actual), expected);
}
