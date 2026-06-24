#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SizeX64 {
    none,
    byte,
    word,
    dword,
    qword,
    xmmword,
    ymmword,
}

#[allow(non_upper_case_globals)]
impl SizeX64 {
    pub const none: Self = Self::none;
    pub const byte: Self = Self::byte;
    pub const word: Self = Self::word;
    pub const dword: Self = Self::dword;
    pub const qword: Self = Self::qword;
    pub const xmmword: Self = Self::xmmword;
    pub const ymmword: Self = Self::ymmword;
}
