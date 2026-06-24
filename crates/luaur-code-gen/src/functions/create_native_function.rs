//! @interface-stub
use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::enums::include_cfg_info::IncludeCfgInfo;
use crate::enums::include_ir_prefix::IncludeIrPrefix;
use crate::enums::include_reg_flow_info::IncludeRegFlowInfo;
use crate::enums::include_use_info::IncludeUseInfo;
use crate::enums::target::Target;
use crate::functions::create_native_proto_exec_data_code_gen_context::create_native_proto_exec_data;
use crate::functions::lower_function::{lower_function_a_64, lower_function_x_64};
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::compilation_options::CompilationOptions;
use crate::records::ir_builder::IrBuilder;
use crate::records::module_helpers::ModuleHelpers;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use luaur_vm::records::proto::Proto;

pub unsafe fn create_native_function_x_64(
    build: &mut AssemblyBuilderX64,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    total_ir_inst_count: &mut u32,
    options: &CompilationOptions,
    result: &mut CodeGenCompilationResult,
) -> Option<NativeProtoExecDataPtr> {
    let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
    ir.build_function_ir(proto);

    let inst_count = ir.function.instructions.len() as u32;

    if total_ir_inst_count.wrapping_add(inst_count)
        >= luaur_common::FInt::CodegenHeuristicsInstructionLimit.get() as u32
    {
        *result = CodeGenCompilationResult::CodeGenOverflowInstructionLimit;
        return None;
    }

    *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

    let assembly_options = AssemblyOptions {
        target: Target::default(),
        compilation_options: options.clone(),
        output_binary: false,
        include_assembly: false,
        include_ir: false,
        include_outlined_code: false,
        include_ir_types: false,
        include_ir_prefix: IncludeIrPrefix::default(),
        include_use_info: IncludeUseInfo::default(),
        include_cfg_info: IncludeCfgInfo::default(),
        include_reg_flow_info: IncludeRegFlowInfo::default(),
        annotator: None,
        annotator_context: core::ptr::null_mut(),
    };

    if !lower_function_x_64(
        &mut ir,
        build,
        helpers,
        proto,
        assembly_options,
        core::ptr::null_mut(),
        result,
    ) {
        return None;
    }

    Some(create_native_proto_exec_data(proto, &ir))
}

pub unsafe fn create_native_function_a_64(
    build: &mut AssemblyBuilderA64,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    total_ir_inst_count: &mut u32,
    options: &CompilationOptions,
    result: &mut CodeGenCompilationResult,
) -> Option<NativeProtoExecDataPtr> {
    let mut ir = IrBuilder::ir_builder_ir_builder(&options.hooks);
    ir.build_function_ir(proto);

    let inst_count = ir.function.instructions.len() as u32;

    if total_ir_inst_count.wrapping_add(inst_count)
        >= luaur_common::FInt::CodegenHeuristicsInstructionLimit.get() as u32
    {
        *result = CodeGenCompilationResult::CodeGenOverflowInstructionLimit;
        return None;
    }

    *total_ir_inst_count = total_ir_inst_count.wrapping_add(inst_count);

    let assembly_options = AssemblyOptions {
        target: Target::default(),
        compilation_options: options.clone(),
        output_binary: false,
        include_assembly: false,
        include_ir: false,
        include_outlined_code: false,
        include_ir_types: false,
        include_ir_prefix: IncludeIrPrefix::default(),
        include_use_info: IncludeUseInfo::default(),
        include_cfg_info: IncludeCfgInfo::default(),
        include_reg_flow_info: IncludeRegFlowInfo::default(),
        annotator: None,
        annotator_context: core::ptr::null_mut(),
    };

    if !lower_function_a_64(
        &mut ir,
        build,
        helpers,
        proto,
        assembly_options,
        core::ptr::null_mut(),
        result,
    ) {
        return None;
    }

    Some(create_native_proto_exec_data(proto, &ir))
}
