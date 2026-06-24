#[derive(Debug, Clone)]
pub struct HoldConditionalExecution {
    pub(crate) reset_to_false: bool,
    pub(crate) p: *mut crate::records::lint_global_local::LintGlobalLocal,
}
