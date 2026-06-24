use crate::functions::destroy_block_unwind_info::destroy_block_unwind_info;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocator::CodeAllocator;
use luaur_common::FFlag;

impl CodeAllocator {
    pub fn drop(&mut self) {
        if self.destroyed {
            return;
        }

        self.destroyed = true;

        if let Some(destroy_block_unwind_info_fn) = self.destroy_block_unwind_info {
            for unwind_info in &self.unwind_infos {
                unsafe {
                    destroy_block_unwind_info_fn(self.context, *unwind_info);
                }
            }
        }

        if FFlag::LuauCodegenFreeBlocks.get() {
            CODEGEN_ASSERT!(self.live_allocations == 0);
        }

        for block in &self.blocks {
            unsafe {
                self.free_pages(*block, self.block_size);
            }
        }

        self.unwind_infos.clear();
        self.blocks.clear();
        self.block_pos = core::ptr::null_mut();
        self.block_end = core::ptr::null_mut();
    }
}
