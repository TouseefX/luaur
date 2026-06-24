use crate::records::fallback_stream_scope::FallbackStreamScope;

impl<'a> FallbackStreamScope<'a> {
    pub fn drop(&mut self) {
        self.build.begin_block(self.next);
    }
}
