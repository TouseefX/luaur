#[allow(non_snake_case)]
use crate::enums::control_flow::ControlFlow;

pub fn operator_bitor(a: ControlFlow, b: ControlFlow) -> ControlFlow {
    ControlFlow::from_bits(a as u32 | b as u32)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitor as operator_bitor_control_flow_control_flow;
