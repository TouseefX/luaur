use alloc::boxed::Box;
use luaur_code_gen::records::host_ir_hooks::HostIrHooks;
use luaur_code_gen::records::ir_builder::IrBuilder;

/// Port of the C++ `IrBuilderFixture` (tests/IrBuilder.test.cpp). The C++
/// `IrBuilder` holds a `const HostIrHooks&`; here the hooks are boxed so their
/// heap address stays stable even if the fixture value is moved (the builder
/// keeps a raw `*const HostIrHooks` into them).
pub struct IrBuilderFixture {
    #[allow(dead_code)]
    pub hooks: Box<HostIrHooks>,
    pub build: IrBuilder,
}

impl Default for IrBuilderFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl IrBuilderFixture {
    pub fn new() -> Self {
        let hooks = Box::new(HostIrHooks::default());
        let build = IrBuilder::ir_builder_ir_builder(&hooks);
        Self { hooks, build }
    }
}
