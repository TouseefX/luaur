use crate::records::cfg_builder::CfgBuilder;
use crate::records::join::Join;

impl CfgBuilder {
    pub fn trim_trivial_join(&mut self, _j: *mut Join) {}
}
