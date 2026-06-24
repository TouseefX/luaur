#[cfg(test)]
#[test]
fn parser_parse_nesting_based_end_detection_nested() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "-- i am line 1
function stringifyTable(t)
    local entries = {}
    for k, v in pairs(t) do
        -- if we find a nested table, convert that recursively
        if type(v) == \"table\" then
            v = stringifyTable(v)
        else
            v = tostring(v)
        k = tostring(k)

        -- add another entry to our stringified table
        entries[#entries + 1] = (\"s = s\"):format(k, v)
    end

    -- the memory location of the table
    local id = tostring(t):sub(8)

    return (\"{s}@s\"):format(table.concat(entries, \", \"), id)
end
",
    );

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 1);
    let msg = result.errors[0].get_message();
    assert_eq!(
        &*msg,
        "Expected 'end' (to close 'function' at line 2), got <eof>; did you forget to close 'else' at line 8?"
    );
}
