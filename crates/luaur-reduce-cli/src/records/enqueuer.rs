use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_visitor::AstVisitor;
use std::collections::VecDeque;

/// `Enqueuer` is a visitor that pushes visited `AstStatBlock`s into a queue.
/// It is native-only and not portable to wasm32-unknown-unknown.
#[repr(C)]
#[derive(Debug)]
pub struct Enqueuer {
    pub queue: *mut VecDeque<*mut AstStatBlock>,
}

impl Enqueuer {
    pub fn new(queue: *mut VecDeque<*mut AstStatBlock>) -> Self {
        // LUAU_ASSERT(queue);
        // SAFETY: The caller must guarantee `queue` is non-null and valid for the lifetime of this Enqueuer.
        unsafe {
            debug_assert!(!queue.is_null());
        }
        Enqueuer { queue }
    }
}

impl AstVisitor for Enqueuer {
    fn visit_stat_block(&mut self, node: *mut core::ffi::c_void) -> bool {
        // SAFETY: `node` is a valid `*mut AstStatBlock` as guaranteed by the visitor dispatch mechanism.
        let block = unsafe { node as *mut AstStatBlock };
        // SAFETY: `self.queue` is non-null as ensured in `new`.
        unsafe {
            (*self.queue).push_back(block);
        }
        false
    }
}
