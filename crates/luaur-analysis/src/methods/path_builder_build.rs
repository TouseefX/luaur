use crate::records::path::Path;
use crate::records::path_builder::PathBuilder;

pub trait PathBuilderBuild {
    fn build(&mut self) -> Path;
}

impl PathBuilderBuild for PathBuilder {
    fn build(&mut self) -> Path {
        Path::from_components(core::mem::take(&mut self.components))
    }
}
