//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2588:type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - translates_to -> rust_item type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type

#[cfg(test)]
#[test]
fn type_infer_txnlog_checks_for_occurrence_before_self_binding_a_type() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);

    let mut fixture = Fixture::default();
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local any = nil :: any

        function f1(x)
            x:m()
            local _ = x.A.p.a
        end

        function f2(x)
            local _ = x.d
        end

        function f3(x)
            local a = ""
            a = x.d.p
            local _ = undef[x.a]
        end

        function f4(x)
            f2(x)
            if undef and x and x:m() then
                any(x)
                return
            end
            f3(x)
            for _, v in any.x do
                local a = x[v].p
            end
            a.b = x
            if x.q ~= nil then
                f1(x) -- things go bad here
            end
        end

        return f4
    "#,
        ),
        None,
    );
}
