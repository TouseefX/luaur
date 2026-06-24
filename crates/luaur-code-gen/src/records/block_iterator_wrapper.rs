#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockIteratorWrapper {
    pub(crate) itBegin: *const u32,
    pub(crate) itEnd: *const u32,
}

impl Default for BlockIteratorWrapper {
    fn default() -> Self {
        Self {
            itBegin: core::ptr::null(),
            itEnd: core::ptr::null(),
        }
    }
}

impl Iterator for BlockIteratorWrapper {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.itBegin < self.itEnd {
            let value = unsafe { *self.itBegin };
            self.itBegin = unsafe { self.itBegin.add(1) };
            Some(value)
        } else {
            None
        }
    }
}
