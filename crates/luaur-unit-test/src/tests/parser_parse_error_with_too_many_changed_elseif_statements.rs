#[cfg(test)]
#[test]
fn parser_parse_error_with_too_many_changed_elseif_statements() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let _sfi = ScopedFastInt::new(&FInt::LuauRecursionLimit, 10);

    let mut fix = Fixture::default();
    fix.match_parse_error_prefix(
        &alloc::string::String::from(
            "function f() if false then elseif false then elseif false then elseif false then \
             elseif false then elseif false then elseif false then elseif false then \
             elseif false then elseif false then elseif false then end end",
        ),
        &alloc::string::String::from("Exceeded allowed recursion depth;"),
    );
}
