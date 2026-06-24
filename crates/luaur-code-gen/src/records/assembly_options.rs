use crate::enums::include_cfg_info::IncludeCfgInfo;
use crate::enums::include_ir_prefix::IncludeIrPrefix;
use crate::enums::include_reg_flow_info::IncludeRegFlowInfo;
use crate::enums::include_use_info::IncludeUseInfo;
use crate::enums::target::Target;
use crate::records::compilation_options::CompilationOptions;
use crate::type_aliases::annotator_fn::AnnotatorFn;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyOptions {
    pub target: Target,
    pub compilation_options: CompilationOptions,
    pub output_binary: bool,
    pub include_assembly: bool,
    pub include_ir: bool,
    pub include_outlined_code: bool,
    pub include_ir_types: bool,
    pub include_ir_prefix: IncludeIrPrefix,
    pub include_use_info: IncludeUseInfo,
    pub include_cfg_info: IncludeCfgInfo,
    pub include_reg_flow_info: IncludeRegFlowInfo,
    pub annotator: AnnotatorFn,
    pub annotator_context: *mut core::ffi::c_void,
}
