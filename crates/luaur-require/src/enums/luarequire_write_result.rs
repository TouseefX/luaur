#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum luarequire_WriteResult {
    WRITE_SUCCESS,
    WRITE_BUFFER_TOO_SMALL,
    WRITE_FAILURE,
}

pub use luarequire_WriteResult as LuarequireWriteResult;

impl luarequire_WriteResult {
    pub const WRITE_SUCCESS: luarequire_WriteResult = luarequire_WriteResult::WRITE_SUCCESS;
    pub const WRITE_BUFFER_TOO_SMALL: luarequire_WriteResult =
        luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
    pub const WRITE_FAILURE: luarequire_WriteResult = luarequire_WriteResult::WRITE_FAILURE;
}
