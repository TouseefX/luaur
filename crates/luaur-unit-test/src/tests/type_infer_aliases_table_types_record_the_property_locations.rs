//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_table_types_record_the_property_locations() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Table = {
            create: () -> ()
        }

        local x: Table
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.require_type_alias(&String::from("Table"));
    let ttv = unsafe { get_type_id::<TableType>(follow_type_id(ty)).as_ref() }
        .expect("expected TableType");
    let prop = ttv.props.get("create").expect("expected create prop");

    assert_eq!(None, prop.location);
    assert_eq!(
        Some(Location {
            begin: Position {
                line: 2,
                column: 12,
            },
            end: Position {
                line: 2,
                column: 18,
            },
        }),
        prop.type_location
    );
}
