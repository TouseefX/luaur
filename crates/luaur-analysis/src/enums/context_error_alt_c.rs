#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Context {
    Property,
    Indexer,
    Metatable,
}

#[allow(non_upper_case_globals)]
impl Context {
    pub const Property: Context = Context::Property;
    pub const Indexer: Context = Context::Indexer;
    pub const Metatable: Context = Context::Metatable;
}
