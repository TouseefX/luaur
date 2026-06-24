#[cfg(test)]
#[test]
fn parser_stat_end_includes_semicolon_position() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        local x = 1\n        local y = 2;\n        local z = 3  ;\n    ",
    );
    let options = ParseOptions::parse_options();
    let block = fixture.parse(&source, &options);

    unsafe {
        assert_eq!(3, (*block).body.size);

        let stat1 = (*block).body.data.add(0);
        assert!(!(*stat1).is_null());
        assert!(!(*stat1).as_ref().unwrap().has_semicolon);
        assert_eq!(
            Position::new(1, 19),
            (*stat1).as_ref().unwrap().base.location.end
        );

        let stat2 = (*block).body.data.add(1);
        assert!(!(*stat2).is_null());
        assert!((*stat2).as_ref().unwrap().has_semicolon);
        assert_eq!(
            Position::new(2, 20),
            (*stat2).as_ref().unwrap().base.location.end
        );

        let stat3 = (*block).body.data.add(2);
        assert!(!(*stat3).is_null());
        assert!((*stat3).as_ref().unwrap().has_semicolon);
        assert_eq!(
            Position::new(3, 22),
            (*stat3).as_ref().unwrap().base.location.end
        );
    }
}
