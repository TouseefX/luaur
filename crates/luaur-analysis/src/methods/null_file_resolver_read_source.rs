use crate::records::null_file_resolver::NullFileResolver;
use crate::records::source_code::SourceCode;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl NullFileResolver {
    pub fn read_source_impl(&self, _name: &ModuleName) -> Option<SourceCode> {
        None
    }
}
