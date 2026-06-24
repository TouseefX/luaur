use crate::enums::navigation_status::NavigationStatus;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct ResolvedRealPath {
    pub(crate) status: NavigationStatus,
    pub(crate) realPath: String,
}

#[allow(non_snake_case)]
impl ResolvedRealPath {
    pub const fn new(status: NavigationStatus, realPath: String) -> Self {
        Self { status, realPath }
    }
}
