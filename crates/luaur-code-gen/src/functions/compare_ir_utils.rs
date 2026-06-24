use crate::enums::ir_condition::IrCondition;
use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn compare_f64_f64_ir_condition(a: f64, b: f64, cond: IrCondition) -> bool {
    // Note: the C++ source uses redundant bool() casts to work around an invalid MSVC
    // optimization that violated IEEE754 comparison semantics; Rust has no such issue.
    match cond {
        IrCondition::Equal => a == b,
        IrCondition::NotEqual => a != b,
        IrCondition::Less => a < b,
        IrCondition::NotLess => !(a < b),
        IrCondition::LessEqual => a <= b,
        IrCondition::NotLessEqual => !(a <= b),
        IrCondition::Greater => a > b,
        IrCondition::NotGreater => !(a > b),
        IrCondition::GreaterEqual => a >= b,
        IrCondition::NotGreaterEqual => !(a >= b),
        _ => {
            CODEGEN_ASSERT!(false, "Unsupported condition");
            false
        }
    }
}
