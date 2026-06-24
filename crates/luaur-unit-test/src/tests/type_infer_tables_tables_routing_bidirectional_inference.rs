//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7079:type_infer_tables_tables_routing_bidirectional_inference`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - translates_to -> rust_item type_infer_tables_tables_routing_bidirectional_inference

#[cfg(test)]
#[test]
fn type_infer_tables_tables_routing_bidirectional_inference() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _better_union_handling =
        ScopedFastFlag::new(&FFlag::LuauBidirectionalInferenceBetterUnionHandling, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type ReceivedRequest = {
            method: string,
            path: string,
            body: string,
            query: { [string]: string },
            headers: { [string]: string },
            params: { [string]: string },
        }

        export type ServerResponse = string | {
            status: number?,
            body: string?,
            headers: { [string]: string }?,
        }

        export type RouteHandler = Handler | ServerResponse

        export type MethodRoutes = {
            GET: RouteHandler?,
            POST: RouteHandler?,
            PUT: RouteHandler?,
            DELETE: RouteHandler?,
            PATCH: RouteHandler?,
            HEAD: RouteHandler?,
            OPTIONS: RouteHandler?,
        }

        export type RouteEntry = RouteHandler | MethodRoutes

        export type Routes = { [string]: RouteEntry }

        export type Server = {
            hostname: string,
            port: number,
            close: () -> (),
            upgrade: (self: Server, req: ReceivedRequest) -> boolean,
        }

        export type Handler = (request: ReceivedRequest, server: Server) -> ServerResponse?

        local routes: Routes? = {
            ["/health"] = "ok",
            ["/json"] = {
                status = 200,
                headers = { ["Content-Type"] = "application/json" },
                body = '{"ok":true}',
            },
            ["/hello"] = function(req)
                local _ = req
                return { status = 200, body = "hello" }
            end,
        }

    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "ReceivedRequest",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(49, 28)))
    );
}
