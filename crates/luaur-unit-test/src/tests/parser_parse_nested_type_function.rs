#[cfg(test)]
#[test]
fn parser_parse_nested_type_function() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
        r#"local v1 = 1
type function foo()
    local v2 = 2
    local function bar()
        v2 += 1
        type function inner() end
        v2 += 2
    end
    local function bar2()
        v2 += 3
    end
end
local function bar() v1 += 1 end"#,
        &ParseOptions::default(),
    );
    assert!(!_stat.is_null());
}
