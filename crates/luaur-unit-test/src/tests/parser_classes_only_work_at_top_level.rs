#[cfg(test)]
#[test]
fn parser_classes_only_work_at_top_level() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from(
            "\n            return function ()\n                class DynamicPlayer\n                    public level: number\n                end\n                return DynamicPlayer\n            end\n        ",
        ),
        &alloc::string::String::from(
            "Cannot declare class 'DynamicPlayer' inside another statement or expression",
        ),
        None,
    );

    fix.match_parse_error(
        &alloc::string::String::from(
            "\n            if math.random() > 0.5 then\n                class DynamicPlayer\n                    public level: number\n                end\n            end\n        ",
        ),
        &alloc::string::String::from(
            "Cannot declare class 'DynamicPlayer' inside another statement or expression",
        ),
        None,
    );
}
