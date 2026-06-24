//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:315:type_infer_provisional_bail_early_if_unification_is_too_complicated`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnificationTooComplex (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_provisional_bail_early_if_unification_is_too_complicated

#[cfg(test)]
#[test]
fn type_infer_provisional_bail_early_if_unification_is_too_complicated() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::unification_too_complex::UnificationTooComplex;
    use luaur_common::FInt;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _tarjan_child_limit = ScopedFastInt::new(&FInt::LuauTarjanChildLimit, 1);
    let _iteration_limit = ScopedFastInt::new(&FInt::LuauTypeInferIterationLimit, 1);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Result
        Result = setmetatable({}, {})
        Result.__index = Result
        function Result.new(okValue)
            local self = setmetatable({}, Result)
            self:constructor(okValue)
            return self
        end
        function Result:constructor(okValue)
            self.okValue = okValue
        end
        function Result:ok(val) return Result.new(val) end
        function Result:a(p0, p1, p2, p3, p4) return Result.new((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:b(p0, p1, p2, p3, p4) return Result:ok((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:c(p0, p1, p2, p3, p4) return Result:ok((self.okValue)) or p0 or p1 or p2 or p3 or p4 end
        function Result:transpose(a)
            return a and self.okValue:z(function(some)
                return Result:ok(some)
            end) or Result:ok(self.okValue)
        end
    "#,
        ),
        None,
    );

    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<UnificationTooComplex>(error).is_some()),
        "{:?}",
        result.errors
    );
}
