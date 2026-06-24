#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CodeAllocationData {
    pub start: *mut u8,
    pub size: usize,
    pub code_start: *mut u8,
    pub allocation_start: *mut u8,
    pub allocation_size: usize,
}

impl Default for CodeAllocationData {
    fn default() -> Self {
        Self {
            start: core::ptr::null_mut(),
            size: 0,
            code_start: core::ptr::null_mut(),
            allocation_start: core::ptr::null_mut(),
            allocation_size: 0,
        }
    }
}
