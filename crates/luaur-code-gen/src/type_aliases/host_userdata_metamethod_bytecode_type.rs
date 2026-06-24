use crate::enums::host_metamethod::HostMetamethod;

#[allow(non_camel_case_types)]
pub type HostUserdataMetamethodBytecodeType =
    Option<unsafe extern "C" fn(lhs_ty: u8, rhs_ty: u8, method: HostMetamethod) -> u8>;
