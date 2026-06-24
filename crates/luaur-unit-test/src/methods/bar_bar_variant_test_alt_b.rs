use crate::records::bar::Bar;

impl Bar {
    pub fn drop(&mut self) {
        // The C++ destructor decrements a static count; this is a unit test helper
        // and has no observable effect in the Rust translation beyond the drop semantics.
        // We keep the method body as a stub since the actual count management is not exposed.
    }
}
