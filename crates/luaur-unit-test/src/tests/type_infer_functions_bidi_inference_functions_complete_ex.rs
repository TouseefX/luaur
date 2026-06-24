//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4139:type_infer_functions_bidi_inference_functions_complete_ex`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item type_infer_functions_bidi_inference_functions_complete_ex

#[cfg(test)]
#[test]
fn type_infer_functions_bidi_inference_functions_complete_ex() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_unions =
        ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);
    let _instantiation = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type Player = {}

        export type RemoteEventWrapper<T...> = {
            connect:( self: RemoteEventWrapper<T...>, callback: ((T...) -> ()) | ((player: Player, T...) -> ()) ) -> () -> (),
        }

        local function useRemoteEvent<T...>(remoteEventName: string, isUnreliable: boolean?): RemoteEventWrapper<T...>
            return nil :: any
        end

        type Payload = {
            name: string,
            time: number,
            data: { [string]: any },
        }

        local payload = useRemoteEvent<<(Payload)>>("initial-payload")

        -- We expect bidirectional inference to kick in here and ensure that
        -- player and payload have non-unknown types.
        payload:connect(function(player, payload)
            local _ = player
            local _ = payload
        end)

        return useRemoteEvent
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Player",
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 23,
            column: 23
        }))
    );
    assert_eq!(
        "Payload",
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 24,
            column: 23
        }))
    );
}
