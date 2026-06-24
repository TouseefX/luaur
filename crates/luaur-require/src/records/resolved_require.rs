use crate::enums::status_require_impl::Status;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedRequire {
    pub(crate) status: Status,
    pub(crate) chunkname: alloc::string::String,
    pub(crate) loadname: alloc::string::String,
    pub(crate) cacheKey: alloc::string::String,
    pub(crate) error: alloc::string::String,
}
