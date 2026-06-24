#[cfg(test)]
#[test]
fn compiler_loop_continue_until_capture() {
    use crate::functions::compile_function::compile_function;
    use luaur_analysis::functions::print::print;
    use luaur_vm::functions::lua_gettop::lua_gettop;
    use luaur_vm::functions::lua_l_tolstring::lua_l_tolstring;
    use luaur_vm::macros::lua_pop::lua_pop;

    // validate continue upvalue closing behavior: continue must close locals defined in the nested scopes
    // but can't close locals defined in the loop scope - these are visible to the condition and will be closed
    // when evaluating the condition instead.
    let actual = compile_function(
        r#"local a a = 0
repeat
    local b b = 0
    if a then
        local c
        print(function() c = 0 end)
        if a then
            continue -- must close c but not a/b
        end
        -- must close c
    end
    -- must close b but not a
until function() a = 0 b = 0 end
-- must close b on loop exit
-- must close a
"#,
        2,
        2,
        0,
    );
    let expected = "\nLOADNIL R0\nLOADN R0 0\nL0: LOADNIL R1\nLOADN R1 0\nJUMPIFNOT R0 L2\nLOADNIL R2\nGETIMPORT R3 1 [print]\nNEWCLOSURE R4 P0\nCAPTURE REF R2\nCALL R3 1 0\nJUMPIFNOT R0 L1\nCLOSEUPVALS R2\nJUMP L2\nL1: CLOSEUPVALS R2\nL2: NEWCLOSURE R2 P1\nCAPTURE REF R0\nCAPTURE REF R1\nJUMPIF R2 L3\nCLOSEUPVALS R1\nJUMPBACK L0\nL3: CLOSEUPVALS R1\nCLOSEUPVALS R0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // a simpler version of the above test doesn't need to close anything when evaluating continue
    let actual = compile_function(
        r#"local a a = 0
repeat
    local b b = 0
    if a then
        continue -- must not close a/b
    end
    -- must close b but not a
until function() a = 0 b = 0 end
-- must close b on loop exit
-- must close a
"#,
        1,
        1,
        0,
    );
    let expected = "\nLOADNIL R0\nLOADN R0 0\nL0: LOADNIL R1\nLOADN R1 0\nJUMPIF R0 L1\nL1: NEWCLOSURE R2 P0\nCAPTURE REF R0\nCAPTURE REF R1\nJUMPIF R2 L2\nCLOSEUPVALS R1\nJUMPBACK L0\nL2: CLOSEUPVALS R1\nCLOSEUPVALS R0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
