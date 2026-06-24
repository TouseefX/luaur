use alloc::vec::Vec;

use luaur_ast::records::ast_stat::AstStat;

use crate::records::reducer::Reducer;

impl Reducer {
    /// Move new body data into allocator-managed storage so that it's safe to keep around
    /// longterm. C++ (`CLI/src/Reduce.cpp:233-239`):
    /// ```cpp
    /// AstStat** newData = static_cast<AstStat**>(allocator.allocate(sizeof(AstStat*) * statements.size()));
    /// std::copy(statements.data(), statements.data() + statements.size(), newData);
    /// return newData;
    /// ```
    /// The pointers are copied into arena storage owned by `self.allocator`, which outlives
    /// the `Reducer`, so the returned pointer stays valid for the rest of the run.
    pub fn reallocate_statements(&mut self, statements: &Vec<*mut AstStat>) -> *mut *mut AstStat {
        let count = statements.len();
        if count == 0 {
            return core::ptr::null_mut();
        }

        let bytes = core::mem::size_of::<*mut AstStat>() * count;
        let new_data = self.allocator.allocate(bytes) as *mut *mut AstStat;

        unsafe {
            core::ptr::copy_nonoverlapping(statements.as_ptr(), new_data, count);
        }

        new_data
    }
}

pub fn reducer_reallocate_statements(
    this: &mut Reducer,
    statements: &Vec<*mut AstStat>,
) -> *mut *mut AstStat {
    this.reallocate_statements(statements)
}
