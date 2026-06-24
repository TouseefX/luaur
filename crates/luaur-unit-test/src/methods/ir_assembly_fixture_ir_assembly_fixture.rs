use crate::records::ir_assembly_fixture::IrAssemblyFixture;
use alloc::boxed::Box;
use luaur_code_gen::enums::include_cfg_info::IncludeCfgInfo;
use luaur_code_gen::enums::include_ir_prefix::IncludeIrPrefix;
use luaur_code_gen::enums::include_reg_flow_info::IncludeRegFlowInfo;
use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
use luaur_code_gen::enums::target::Target;
use luaur_code_gen::records::assembly_options::AssemblyOptions;
use luaur_code_gen::records::compilation_options::CompilationOptions;
use luaur_code_gen::records::host_ir_hooks::HostIrHooks;
use luaur_code_gen::records::ir_builder::IrBuilder;

impl IrAssemblyFixture {
    pub fn ir_assembly_fixture() -> Self {
        let hooks = Box::new(HostIrHooks::default());
        let build = IrBuilder::ir_builder_ir_builder(&hooks);
        let options = AssemblyOptions {
            target: Target::X64_Windows,
            compilation_options: CompilationOptions::default(),
            output_binary: false,
            include_assembly: true,
            include_ir: true,
            include_outlined_code: false,
            include_ir_types: true,
            include_ir_prefix: IncludeIrPrefix::No,
            include_use_info: IncludeUseInfo::No,
            include_cfg_info: IncludeCfgInfo::No,
            include_reg_flow_info: IncludeRegFlowInfo::No,
            annotator: None,
            annotator_context: core::ptr::null_mut(),
        };

        Self {
            hooks,
            build,
            options,
        }
    }
}
