use crate::enums::condition_a_64::ConditionA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;

impl AssemblyBuilderA64 {
    pub fn b_condition_a_64_label(&mut self, cond: ConditionA64, label: &mut Label) {
        let name = match cond {
            ConditionA64::Equal => b"b.eq\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::NotEqual => b"b.ne\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::CarrySet => b"b.cs\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::CarryClear => b"b.cc\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Minus => b"b.mi\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Plus => b"b.pl\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Overflow => b"b.vs\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::NoOverflow => b"b.vc\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::UnsignedGreater => b"b.hi\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::UnsignedLessEqual => b"b.ls\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::GreaterEqual => b"b.ge\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Less => b"b.lt\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Greater => b"b.gt\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::LessEqual => b"b.le\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Always => b"b\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::Count => b"b\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::UnsignedLess => b"b\0".as_ptr() as *const core::ffi::c_char,
            ConditionA64::UnsignedGreaterEqual => b"b\0".as_ptr() as *const core::ffi::c_char,
        };
        let code = match cond {
            ConditionA64::Equal => 0b0101010_000000u32,
            ConditionA64::NotEqual => 0b0101010_000001u32,
            ConditionA64::CarrySet => 0b0101010_000010u32,
            ConditionA64::CarryClear => 0b0101010_000011u32,
            ConditionA64::Minus => 0b0101010_000100u32,
            ConditionA64::Plus => 0b0101010_000101u32,
            ConditionA64::Overflow => 0b0101010_000110u32,
            ConditionA64::NoOverflow => 0b0101010_000111u32,
            ConditionA64::UnsignedGreater => 0b0101010_001000u32,
            ConditionA64::UnsignedLessEqual => 0b0101010_001001u32,
            ConditionA64::GreaterEqual => 0b0101010_001010u32,
            ConditionA64::Less => 0b0101010_001011u32,
            ConditionA64::Greater => 0b0101010_001100u32,
            ConditionA64::LessEqual => 0b0101010_001101u32,
            ConditionA64::Always => 0b0101010_111111u32,
            ConditionA64::Count => 0b0101010_111111u32,
            ConditionA64::UnsignedLess => 0b0101010_111111u32,
            ConditionA64::UnsignedGreaterEqual => 0b0101010_111111u32,
        };
        self.place_bc(name, label, 0b0101010_0, (code & 0x3F) as u8);
    }
}
