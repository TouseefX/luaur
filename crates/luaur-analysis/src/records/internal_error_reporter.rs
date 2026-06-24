use luaur_ast::records::location::Location;

#[derive(Clone)]
pub struct InternalErrorReporter {
    pub on_internal_error: Option<alloc::rc::Rc<dyn Fn(&str)>>,
    pub module_name: alloc::string::String,
}

impl Default for InternalErrorReporter {
    fn default() -> Self {
        Self {
            on_internal_error: None,
            module_name: alloc::string::String::new(),
        }
    }
}

impl core::fmt::Debug for InternalErrorReporter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InternalErrorReporter")
            .field(
                "on_internal_error",
                &self.on_internal_error.as_ref().map(|_| "..."),
            )
            .field("module_name", &self.module_name)
            .finish()
    }
}
