use crate::records::cli_file_resolver::CliFileResolver;
use luaur_analysis::enums::type_file_resolver::Type;
use luaur_analysis::records::source_code::SourceCode;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_cli_lib::functions::read_file::read_file;
use luaur_cli_lib::functions::read_stdin::read_stdin;

impl CliFileResolver {
    pub unsafe fn read_source(&mut self, name: &ModuleName) -> Option<SourceCode> {
        let source_type: Type;
        let source: Option<alloc::string::String>;

        // If the module name is "-", then read source from stdin
        if name == "-" {
            source = read_stdin();
            source_type = Type::Script;
        } else {
            source = read_file(name);
            source_type = Type::Module;
        }

        if let Some(source_str) = source {
            Some(SourceCode {
                source: source_str,
                r#type: source_type,
            })
        } else {
            None
        }
    }
}
