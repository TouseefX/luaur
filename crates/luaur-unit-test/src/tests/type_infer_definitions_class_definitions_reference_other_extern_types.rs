//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:511:type_infer_definitions_class_definitions_reference_other_extern_types`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_definitions_class_definitions_reference_other_extern_types

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definitions_reference_other_extern_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Channel
            Messages: { Message }
            OnMessage: (message: Message) -> ()
        end

        declare class Message
            Text: string
            Channel: Channel
        end
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: Channel
        local b = a.Messages[1]
        local c = b.Channel
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Channel",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "Message",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "Channel",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
}
