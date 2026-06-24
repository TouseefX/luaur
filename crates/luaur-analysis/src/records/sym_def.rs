use crate::records::symbol::Symbol;
use alloc::string::String;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct SymDef {
    pub sym: Symbol,
    pub version: usize,
}

impl PartialEq for SymDef {
    fn eq(&self, other: &Self) -> bool {
        self.sym.operator_eq_symbol(&other.sym) && self.version == other.version
    }
}

impl Eq for SymDef {}

impl core::hash::Hash for SymDef {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.sym.hash_luau_symbol_operator_call().hash(state);
        self.version.hash(state);
    }
}

#[allow(non_snake_case)]
impl SymDef {
    pub fn sym_def(sym: Symbol, version: usize) -> Self {
        Self { sym, version }
    }

    pub fn name(&self) -> String {
        unsafe {
            let cstr = self.sym.c_str();
            core::ffi::CStr::from_ptr(cstr)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn versioned_name(&self) -> String {
        format!("{}-{}", self.name(), self.version)
    }

    pub fn operator_eq_sym_def(&self, other: &Self) -> bool {
        self.sym.operator_eq_symbol(&other.sym) && self.version == other.version
    }

    pub fn operator_ne_sym_def(&self, other: &Self) -> bool {
        !self.operator_eq_sym_def(other)
    }
}

unsafe impl Send for SymDef {}
unsafe impl Sync for SymDef {}
