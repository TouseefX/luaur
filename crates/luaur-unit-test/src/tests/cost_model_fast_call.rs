//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/CostModel.test.cpp:118:cost_model_fast_call`
//! Source: `tests/CostModel.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/CostModel.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/CostModel.test.cpp
//! - outgoing:
//!   - calls -> method CostVisitor::model (Compiler/src/CostModel.cpp)
//!   - calls -> function modelFunction (tests/CostModel.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function computeCost (Compiler/src/CostModel.cpp)
//!   - translates_to -> rust_item cost_model_fast_call

#[cfg(test)]
#[test]
fn cost_model_fast_call() {
    use crate::functions::model_function::model_function;
    use luaur_compiler::functions::compute_cost::compute_cost;
    use std::ffi::CString;

    let source = CString::new(
        r#"
function test(a)
    return math.abs(a + 1)
end
"#,
    )
    .unwrap();
    let model = model_function(source.as_ptr());

    let args1 = [false];
    let args2 = [true];

    assert_eq!(6, compute_cost(model, args1.as_ptr(), 1));
    assert_eq!(5, compute_cost(model, args2.as_ptr(), 1));
}
