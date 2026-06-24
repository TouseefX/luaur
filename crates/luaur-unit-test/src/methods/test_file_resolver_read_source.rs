use crate::records::test_file_resolver::TestFileResolver;
use luaur_analysis::records::source_code::SourceCode;

impl TestFileResolver {
    pub fn read_source(&mut self, name: &str) -> Option<SourceCode> {
        let source = self.source.get(name)?.clone();
        let source_type = self
            .source_types
            .get(name)
            .copied()
            .unwrap_or(SourceCode::Module);

        Some(SourceCode {
            source,
            r#type: source_type,
        })
    }
}
