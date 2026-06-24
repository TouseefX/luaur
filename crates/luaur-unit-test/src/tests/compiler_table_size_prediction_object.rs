#[cfg(test)]
#[test]
fn compiler_table_size_prediction_object() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local t = {}
t.field = 1
function t:getfield()
    return self.field
end
return t"#,
        1,
        1,
        0,
    );
    let expected = "\nNEWTABLE R0 2 0\nLOADN R1 1\nSETTABLEKS R1 R0 K0 ['field']\nDUPCLOSURE R1 K1 ['getfield']\nSETTABLEKS R1 R0 K2 ['getfield']\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);
}
