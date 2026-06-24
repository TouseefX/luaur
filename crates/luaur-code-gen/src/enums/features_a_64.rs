#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FeaturesA64 {
    Feature_JSCVT = 1 << 0,
    Feature_AdvSIMD = 1 << 1,
}

impl FeaturesA64 {
    pub const Feature_JSCVT: FeaturesA64 = FeaturesA64::Feature_JSCVT;
    pub const Feature_AdvSIMD: FeaturesA64 = FeaturesA64::Feature_AdvSIMD;
}
