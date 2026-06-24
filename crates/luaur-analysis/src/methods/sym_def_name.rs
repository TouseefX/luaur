use crate::records::sym_def::SymDef;

impl SymDef {
    pub fn sym_def_name(&self) -> alloc::string::String {
        self.name()
    }
}
