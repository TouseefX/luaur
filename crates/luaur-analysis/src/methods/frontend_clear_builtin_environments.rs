use crate::records::frontend::Frontend;

impl Frontend {
    pub fn clear_builtin_environments(&mut self) {
        self.environments.clear();
    }
}
