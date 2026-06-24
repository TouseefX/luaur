#[cfg(test)]
#[test]
fn parser_dont_parse_attributes_on_non_function_stat() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fix = Fixture::default();

    let pr1 = fix.try_parse(
        &String::from("\n@checked\nif a<0 then a = 0 end"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'if' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'if' instead"
    };
    check_first_error_for_attributes(&pr1.errors, 1, expected_location, expected_message);

    let pr2 = fix.try_parse(
        &String::from(
            "\nlocal i = 1\n@checked\nwhile a[i] do\n    print(a[i])\n    i = i + 1\nend",
        ),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(3, 0), Position::new(3, 5));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'while' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'while' instead"
    };
    check_first_error_for_attributes(&pr2.errors, 1, expected_location, expected_message);

    let pr3 = fix.try_parse(
        &String::from("\n@checked\ndo\n    local a2 = 2*a\n    local d = sqrt(b^2 - 4*a*c)\n    x1 = (-b + d)/a2\n    x2 = (-b - d)/a2\nend"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 2));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'do' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'do' instead"
    };
    check_first_error_for_attributes(&pr3.errors, 1, expected_location, expected_message);

    let pr4 = fix.try_parse(
        &String::from("\n@checked\nfor i=1,10 do print(i) end\n"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 3));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'for' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'for' instead"
    };
    check_first_error_for_attributes(&pr4.errors, 1, expected_location, expected_message);

    let pr5 = fix.try_parse(
        &String::from("\n@checked\nrepeat\n    line = io.read()\nuntil line ~= \"\"\n"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 0), Position::new(2, 6));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'repeat' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'repeat' instead"
    };
    check_first_error_for_attributes(&pr5.errors, 1, expected_location, expected_message);

    let pr6 = fix.try_parse(
        &String::from("\n@checked\nlocal x = 10\n"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 6), Position::new(2, 7));
    check_first_error_for_attributes(
        &pr6.errors,
        1,
        expected_location,
        "Expected 'function' after local declaration with attribute, but got 'x' instead",
    );

    // C++: ScopedFastFlag sffs[] = {{LuauExportValueSyntax, true}, {LuauConst2, true}};
    // These live through pr7..pr9; without LuauExportValueSyntax the `export local`
    // case below takes a different parse path and reports a different error.
    let _sff_export = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let _sff_const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);

    let pr7 = fix.try_parse(
        &String::from("\n@checked\nexport local x = 10\n"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(2, 7), Position::new(2, 12));
    check_first_error_for_attributes(
        &pr7.errors,
        1,
        expected_location,
        "Expected 'function' after export declaration with attribute, but got 'local' instead",
    );

    let pr8 = fix.try_parse(
        &String::from(
            "\nlocal i = 1\nwhile a[i] do\n    if a[i] == v then @checked break end\n    i = i + 1\nend\n",
        ),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(3, 31), Position::new(3, 36));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'break' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'break' instead"
    };
    check_first_error_for_attributes(&pr8.errors, 1, expected_location, expected_message);

    let pr9 = fix.try_parse(
        &String::from("\nfunction foo1 () @checked return 'a' end\n"),
        &ParseOptions::default(),
    );
    let expected_location = Location::new(Position::new(1, 26), Position::new(1, 32));
    let expected_message = if FFlag::LuauConst2.get() {
        "Expected 'function', 'local function', 'const function', 'declare function' or a function type declaration after attribute, but got 'return' instead"
    } else {
        "Expected 'function', 'local function', 'declare function' or a function type declaration after attribute, but got 'return' instead"
    };
    check_first_error_for_attributes(&pr9.errors, 1, expected_location, expected_message);
}
