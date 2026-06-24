//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_react_use_state_partial_annotation() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type BasicStateAction<S> = ((S) -> S) | S
        type Dispatch<A> = (A) -> ()

        local useState: <S>( (() -> S) | S ) -> (S, Dispatch<BasicStateAction<S>>) = nil :: any

        local v: number, setV = useState(0)
        local w, setW = useState(0 :: number?)
        local x, setX = useState(0)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(((number) -> number) | number) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("setV")))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("w")))
    );
    assert_eq!(
        "((((number?) -> number?) | number)?) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("setW")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "(((number) -> number) | number) -> ()",
        to_string_type_id(fixture.base.require_type_string(&String::from("setX")))
    );
}
