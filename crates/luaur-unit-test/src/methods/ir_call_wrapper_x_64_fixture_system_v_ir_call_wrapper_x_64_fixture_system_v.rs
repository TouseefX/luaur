use crate::records::ir_call_wrapper_x_64_fixture_system_v::IrCallWrapperX64FixtureSystemV;
use luaur_code_gen::enums::abix_64::ABIX64;

impl IrCallWrapperX64FixtureSystemV {
    pub fn new() -> Self {
        Self {
            base: crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture::new(
                ABIX64::SystemV,
            ),
        }
    }
}
