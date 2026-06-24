#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct RuntimeLuauConfigTimer {
    pub(crate) start_time: std::time::Instant,
    pub(crate) timeout_duration: Option<std::time::Duration>,
}
