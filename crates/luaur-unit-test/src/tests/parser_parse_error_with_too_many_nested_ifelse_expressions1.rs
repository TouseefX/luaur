#[cfg(test)]
#[test]
fn parser_parse_error_with_too_many_nested_ifelse_expressions1() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let mut fix = Fixture::fixture_bool(false);
    let _sfis = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let source = "function f() return if true then 1 elseif true then 2 elseif true then 3 elseif true then 4 elseif true then 5 elseif true then 6 elseif true then 7 elseif true then 8 elseif true then 9 elseif true then 10 else 11 end";
    let message =
        "Exceeded allowed recursion depth; simplify your expression to make the code compile";
    fix.match_parse_error(&source.to_string(), &message.to_string(), None);
}
