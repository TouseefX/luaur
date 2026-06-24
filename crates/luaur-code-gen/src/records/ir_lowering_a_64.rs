use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::exit_handler_ir_lowering_a_64::ExitHandler;
use crate::records::interrupt_handler_ir_lowering_a_64::InterruptHandler;
use crate::records::ir_block::IrBlock;
use crate::records::ir_const::IrConst;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_a_64::IrRegAllocA64;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;
use crate::records::label::Label;
use crate::records::lowering_stats::LoweringStats;
use crate::records::module_helpers::ModuleHelpers;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrLoweringA64 {
    pub build: *mut AssemblyBuilderA64,
    pub helpers: *mut ModuleHelpers,
    pub function: *mut IrFunction,
    pub stats: *mut LoweringStats,
    pub regs: IrRegAllocA64,
    pub value_tracker: IrValueLocationTracking,
    pub interrupt_handlers: alloc::vec::Vec<InterruptHandler>,
    pub exit_handlers: alloc::vec::Vec<ExitHandler>,
    pub exit_handler_map: DenseHashMap<u32, u32>,
    pub exit_sync_alloc_token: u32,
    pub exit_sync_inst_idx: u32,
    pub error: bool,
}

impl IrLoweringA64 {
    pub const kInvalidInstIdx: u32 = !0u32;

    #[allow(dead_code)]
    fn interrupt_handler_field_shape(self_: Label, pcpos: u32, next: Label) -> InterruptHandler {
        InterruptHandler {
            self_: self_,
            pcpos: pcpos,
            next: next,
        }
    }

    #[allow(dead_code)]
    fn exit_handler_field_shape(self_: Label, pcpos: u32) -> ExitHandler {
        ExitHandler {
            self_: self_,
            pcpos: pcpos,
        }
    }
}
