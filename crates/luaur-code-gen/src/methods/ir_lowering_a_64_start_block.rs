use crate::enums::code_gen_counter::CodeGenCounter;
use crate::enums::ir_block_kind::IrBlockKind;
use crate::records::ir_block::kBlockNoStartPc;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use luaur_common::FFlag;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_start_block(&mut self, curr: &IrBlock) {
        if curr.startpc != kBlockNoStartPc {
            let counter = if curr.kind == IrBlockKind::Fallback {
                CodeGenCounter::FallbackBlockExecuted
            } else {
                CodeGenCounter::RegularBlockExecuted
            };
            self.ir_lowering_a_64_alloc_and_increment_counter_at(counter, curr.startpc);
        }

        if FFlag::LuauCodegenVmExitSync.get() && curr.kind == IrBlockKind::ExitSync {
            let block_index = unsafe { (*self.function).get_block_index(curr) };
            self.regs.setup_exit_sync_entry(block_index);
        }
    }
}
