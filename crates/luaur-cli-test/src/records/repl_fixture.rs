extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::string::String;

use crate::records::completion::Completion;

pub type CompletionSet = BTreeSet<Completion>;

#[derive(Debug, Clone)]
pub struct ReplFixture {
    pub(crate) lua_state: *mut core::ffi::c_void,
    pub(crate) l: *mut core::ffi::c_void,
    pub(crate) pretty_print_source: String,
}
