use crate::records::to_string_span::ToStringSpan;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Default)]
pub struct ElementResult {
    pub str: alloc::string::String,
    pub spans: alloc::vec::Vec<ToStringSpan>,
}

unsafe impl Send for ElementResult {}
unsafe impl Sync for ElementResult {}
