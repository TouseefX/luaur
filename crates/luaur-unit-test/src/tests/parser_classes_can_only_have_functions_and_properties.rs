#[cfg(test)]
#[test]
fn parser_classes_can_only_have_functions_and_properties() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let source = r#"
        class Bicycle
            while true do
                cycle()
            end
        end
    "#;

    let expected_message = "Only class properties and functions can be declared within a class";

    let mut fixture = Fixture::fixture_bool(false);
    let _result =
        fixture.match_parse_error(&source.to_string(), &expected_message.to_string(), None);
}
