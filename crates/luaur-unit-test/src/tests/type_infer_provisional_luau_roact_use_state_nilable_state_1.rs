//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1138:type_infer_provisional_luau_roact_use_state_nilable_state_1`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_provisional_luau_roact_use_state_nilable_state_1

#[cfg(test)]
#[test]
fn type_infer_provisional_luau_roact_use_state_nilable_state_1() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Dispatch<A> = (A) -> ()
        type BasicStateAction<S> = ((S) -> S) | S

        type ScriptConnection = { Disconnect: (ScriptConnection) -> () }

        local blah = nil :: any

        local function useState<S>(
            initialState: (() -> S) | S,
            ...
        ): (S, Dispatch<BasicStateAction<S>>)
            return blah, blah
        end

        local a, b = useState(nil :: ScriptConnection?)

        if a then
            a:Disconnect()
            b(nil :: ScriptConnection?)
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            Location::new(Position::new(19, 14), Position::new(19, 41)),
            result.errors[0].location
        );
    }
}
