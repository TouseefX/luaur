use crate::enums::test_result::TestResult;
use crate::records::reducer::Reducer;
use alloc::vec::Vec;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl Reducer {
    pub fn try_promoting_child_statements_ast_stat_block_usize(
        &mut self,
        b: *mut AstStatBlock,
        index: usize,
    ) -> bool {
        unsafe {
            let body_slice = core::slice::from_raw_parts((*b).body.data, (*b).body.size);
            let mut temp_stats = Vec::from(body_slice);

            let removed = temp_stats[index];
            temp_stats.remove(index);

            let nested_stats = self.get_nested_stats(removed);
            temp_stats.splice(index..index, nested_stats);

            let mut temp_array = AstArray {
                data: temp_stats.as_mut_ptr(),
                size: temp_stats.len(),
            };

            core::mem::swap(&mut (*b).body, &mut temp_array);

            let result = self.run();

            if result == TestResult::BugFound {
                (*b).body.data = self.reallocate_statements(&temp_stats);
                (*b).body.size = temp_stats.len();
                true
            } else {
                core::mem::swap(&mut (*b).body, &mut temp_array);
                false
            }
        }
    }
}
