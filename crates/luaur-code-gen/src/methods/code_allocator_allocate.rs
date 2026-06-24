use crate::functions::flush_instruction_cache_code_allocator::flush_instruction_cache;
use crate::functions::make_pages_executable_code_allocator::make_pages_executable;
use crate::functions::make_pages_read_only_code_allocator::make_pages_read_only;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocation_data::CodeAllocationData;
use crate::records::code_allocator::CodeAllocator;
use core::ffi::c_void;
use core::ptr;

impl CodeAllocator {
    pub fn allocate(
        &mut self,
        data: *const u8,
        data_size: usize,
        code: *const u8,
        code_size: usize,
    ) -> CodeAllocationData {
        use luaur_common::FFlag;
        CODEGEN_ASSERT!(FFlag::LuauCodegenFreeBlocks.get());

        let mut start_offset = 0;
        let code_offset: usize;
        let data_offset: usize;
        let page_aligned_size: usize;
        let total_size: usize;

        if FFlag::LuauCodegenProtectData.get() {
            if data_size != 0 {
                if CodeAllocator::align_to_page_size(Self::kMaxReservedDataSize + data_size)
                    + code_size
                    > self.block_size
                {
                    return CodeAllocationData::default();
                }

                if CodeAllocator::align_to_page_size(data_size) + code_size
                    > (self.block_end as usize - self.block_pos as usize)
                {
                    if !self.allocate_new_block(&mut start_offset) {
                        return CodeAllocationData::default();
                    }
                    CODEGEN_ASSERT!(
                        CodeAllocator::align_to_page_size(start_offset + data_size) + code_size
                            <= (self.block_end as usize - self.block_pos as usize)
                    );
                }

                code_offset = CodeAllocator::align_to_page_size(start_offset + data_size);
                data_offset = code_offset - data_size;
                total_size = CodeAllocator::align_to_page_size(data_size) + code_size;
                page_aligned_size = CodeAllocator::align_to_page_size(code_offset + code_size);
            } else {
                let ts = code_size;
                if ts > self.block_size - Self::kMaxReservedDataSize {
                    return CodeAllocationData::default();
                }

                if ts > (self.block_end as usize - self.block_pos as usize) {
                    if !self.allocate_new_block(&mut start_offset) {
                        return CodeAllocationData::default();
                    }
                    CODEGEN_ASSERT!(ts <= (self.block_end as usize - self.block_pos as usize));
                }

                data_offset = start_offset;
                code_offset = start_offset;
                page_aligned_size = CodeAllocator::align_to_page_size(start_offset + ts);
                total_size = ts;
            }
        } else {
            let k_code_alignment = 32;
            let aligned_data_size = (data_size + (k_code_alignment - 1)) & !(k_code_alignment - 1);
            let ts = aligned_data_size + code_size;

            if ts > self.block_size - Self::kMaxReservedDataSize {
                return CodeAllocationData::default();
            }

            if ts > (self.block_end as usize - self.block_pos as usize) {
                if !self.allocate_new_block(&mut start_offset) {
                    return CodeAllocationData::default();
                }
                CODEGEN_ASSERT!(ts <= (self.block_end as usize - self.block_pos as usize));
            }

            data_offset = start_offset + aligned_data_size - data_size;
            code_offset = start_offset + aligned_data_size;
            page_aligned_size = CodeAllocator::align_to_page_size(start_offset + ts);
            total_size = ts;
        }

        CODEGEN_ASSERT!(
            CodeAllocator::align_to_page_size(self.block_pos as usize) == self.block_pos as usize
        );

        if data_size != 0 {
            unsafe {
                ptr::copy_nonoverlapping(data, self.block_pos.add(data_offset), data_size);
            }
        }
        if code_size != 0 {
            unsafe {
                ptr::copy_nonoverlapping(code, self.block_pos.add(code_offset), code_size);
            }
        }

        if FFlag::LuauCodegenProtectData.get() {
            if data_size != 0 {
                if !make_pages_read_only(self.block_pos, code_offset) {
                    return CodeAllocationData::default();
                }
                if !make_pages_executable(
                    unsafe { self.block_pos.add(code_offset) },
                    page_aligned_size - code_offset,
                ) {
                    return CodeAllocationData::default();
                }
            } else if !make_pages_executable(self.block_pos, page_aligned_size) {
                return CodeAllocationData::default();
            }
        } else if !make_pages_executable(self.block_pos, page_aligned_size) {
            return CodeAllocationData::default();
        }

        self.live_allocations += 1;
        flush_instruction_cache(unsafe { self.block_pos.add(code_offset) }, code_size);

        let result = CodeAllocationData {
            start: unsafe { self.block_pos.add(start_offset) },
            size: total_size,
            code_start: unsafe { self.block_pos.add(code_offset) },
            allocation_start: self.block_pos,
            allocation_size: page_aligned_size,
        };

        if page_aligned_size <= (self.block_end as usize - self.block_pos as usize) {
            self.block_pos = unsafe { self.block_pos.add(page_aligned_size) };
            CODEGEN_ASSERT!(
                CodeAllocator::align_to_page_size(self.block_pos as usize)
                    == self.block_pos as usize
            );
            CODEGEN_ASSERT!(self.block_pos <= self.block_end);
        } else {
            self.block_pos = self.block_end;
        }

        result
    }
}
