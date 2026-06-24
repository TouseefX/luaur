use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub(crate) local: *mut AstLocal,
    pub(crate) global: AstName,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            local: core::ptr::null_mut(),
            global: AstName::new(),
        }
    }
}

impl Symbol {
    pub fn from_local(local: *mut AstLocal) -> Self {
        Self {
            local,
            global: AstName::new(),
        }
    }

    pub fn from_global(global: AstName) -> Self {
        Self {
            local: core::ptr::null_mut(),
            global,
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, rhs: &Self) -> bool {
        if !self.local.is_null() {
            self.local == rhs.local
        } else if !self.global.value.is_null() {
            !rhs.global.value.is_null()
                && unsafe {
                    core::ffi::CStr::from_ptr(self.global.value)
                        == core::ffi::CStr::from_ptr(rhs.global.value)
                }
        } else {
            rhs.local.is_null() && rhs.global.value.is_null()
        }
    }
}

impl Eq for Symbol {}

impl core::hash::Hash for Symbol {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.local.hash(state);

        if !self.global.value.is_null() {
            unsafe {
                core::ffi::CStr::from_ptr(self.global.value)
                    .to_bytes()
                    .hash(state)
            };
        }
    }
}

unsafe impl Send for Symbol {}
unsafe impl Sync for Symbol {}
