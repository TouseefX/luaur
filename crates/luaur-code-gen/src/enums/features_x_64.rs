#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FeaturesX64 {
    Feature_FMA3 = 1 << 0,
    Feature_AVX = 1 << 1,
}

impl FeaturesX64 {
    pub const Feature_FMA3: FeaturesX64 = FeaturesX64::Feature_FMA3;
    pub const Feature_AVX: FeaturesX64 = FeaturesX64::Feature_AVX;
}
