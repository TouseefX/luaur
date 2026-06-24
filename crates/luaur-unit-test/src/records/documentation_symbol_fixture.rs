use crate::records::builtins_fixture::BuiltinsFixture;

#[derive(Debug)]
pub struct DocumentationSymbolFixture {
    pub base: BuiltinsFixture,
}

impl Default for DocumentationSymbolFixture {
    fn default() -> Self {
        Self {
            base: BuiltinsFixture::default(),
        }
    }
}
