#[cfg(test)]
#[test]
fn compiler_fast_call_import_fallback() {
    use crate::functions::compile_function_0::compile_function_0;
    use luaur_common::functions::format_append::formatAppend;
    use luaur_common::functions::split::split;

    let mut source = "local t = {}\n".to_string();

    // we need to exhaust the 10-bit constant space to block GETIMPORT from being emitted
    for i in 1..=1024 {
        formatAppend(&mut source, format_args!("t[{}] = \"{}\"\n", i, i));
    }

    source += "return math.abs(-1)\n";

    let code = compile_function_0(&source);

    let insns: Vec<&str> = split(&code, '\n');

    let mut fragment = String::new();
    for i in (2..=9).rev() {
        fragment += insns[insns.len() - i];
        fragment += "\n";
    }

    let expected = "\nLOADN R1 1024\nLOADK R2 K1023 ['1024']\nSETTABLE R2 R0 R1\nLOADN R2 -1\nFASTCALL1 2 R2 L0\nGETGLOBAL R1 K1024 ['math']\nGETTABLEKS R1 R1 K1025 ['abs']\nCALL R1 1 -1\n";
    assert_eq!("\n".to_string() + &fragment, expected);
}
