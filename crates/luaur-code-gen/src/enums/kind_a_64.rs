#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KindA64 {
    none,
    w, // 32-bit GPR
    x, // 64-bit GPR
    s, // 32-bit SIMD&FP scalar
    d, // 64-bit SIMD&FP scalar
    q, // 128-bit SIMD&FP vector
}

#[allow(non_upper_case_globals)]
impl KindA64 {
    pub const none: Self = Self::none;
    pub const w: Self = Self::w;
    pub const x: Self = Self::x;
    pub const s: Self = Self::s;
    pub const d: Self = Self::d;
    pub const q: Self = Self::q;
}
