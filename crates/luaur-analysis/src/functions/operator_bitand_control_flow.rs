use crate::enums::control_flow::ControlFlow;

#[allow(non_snake_case)]
pub fn operator_bitand(a: ControlFlow, b: ControlFlow) -> ControlFlow {
    ControlFlow::from_bits(a as u32 & b as u32)
}

impl core::ops::BitAnd for ControlFlow {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        operator_bitand(self, rhs)
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitand as operator_bitand_control_flow_control_flow;
