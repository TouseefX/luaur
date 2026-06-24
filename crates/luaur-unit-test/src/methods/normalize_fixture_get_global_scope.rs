//! @interface-stub
use crate::records::normalize_fixture::NormalizeFixture;
use alloc::sync::Arc;
use luaur_analysis::records::scope::Scope;

impl NormalizeFixture {
    pub fn get_global_scope(&mut self) -> *mut Scope {
        self.get_frontend();
        self.global_scope
            .as_ref()
            .map(|scope| Arc::as_ptr(scope) as *mut Scope)
            .unwrap_or(core::ptr::null_mut())
    }
}
