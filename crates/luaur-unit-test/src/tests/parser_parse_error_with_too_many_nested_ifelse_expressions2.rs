#[cfg(test)]
#[test]
fn parser_parse_error_with_too_many_nested_ifelse_expressions2() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let mut fix = Fixture::fixture_bool(false);
    let _scoped = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let source = "function f() return if if if if if if if if if if true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then false else true then 1 else 2 end";
    let message =
        "Exceeded allowed recursion depth; simplify your expression to make the code compile";

    fix.match_parse_error(&source.to_string(), &message.to_string(), None);
}
