#[cfg(test)]
#[test]
fn compiler_inline_only_remove_terminating_jump() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let actual = compile_function(
        r#"local props = {}
local changes = {}

local function perform(name, valueType, updateFunction)
    local valueObj = script:FindFirstChild(name)

    if valueObj then
        props[name] = valueObj.Value
    else
        return
    end

    if updateFunction then
        changes[name] = valueObj.Changed:Connect(function(newValue)
            props[name] = newValue
            updateFunction()
        end)
    end
end

local function performAll()
    perform("InitialElevation", "NumberValue", nil)
    perform("InitialDistance", "NumberValue", nil)

    print("done");
end
"#,
        2,
        2,
        0,
    );

    let expected = r#"
GETIMPORT R0 1 [script]
LOADK R2 K2 ['InitialElevation']
NAMECALL R0 R0 K3 ['FindFirstChild']
CALLFB R0 2 1 [0]
JUMPIFNOT R0 L0
GETUPVAL R1 0
GETTABLEKS R2 R0 K4 ['Value']
SETTABLEKS R2 R1 K2 ['InitialElevation']
JUMP L0
JUMP L0
L0: GETIMPORT R0 1 [script]
LOADK R2 K5 ['InitialDistance']
NAMECALL R0 R0 K3 ['FindFirstChild']
CALLFB R0 2 1 [1]
JUMPIFNOT R0 L1
GETUPVAL R1 0
GETTABLEKS R2 R0 K4 ['Value']
SETTABLEKS R2 R1 K5 ['InitialDistance']
JUMP L1
JUMP L1
L1: GETIMPORT R0 7 [print]
LOADK R1 K8 ['done']
CALLFB R0 1 0 [2]
RETURN R0 0
"#;

    assert_eq!(format!("\n{}", actual), expected);
}
