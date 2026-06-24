use crate::enums::kind_a_64::KindA64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterA64 {
    pub(crate) bits: u8,
}

// The C++ `RegisterA64` is a `{uint8_t index:5; KindA64 kind:3;}` bitfield; this
// port packs both into one `bits` byte (kind low 3, index high 5). `make`
// reproduces the `inline constexpr RegisterA64 name{kind, index}` globals from
// RegisterA64.h, which the extractor does not emit as nodes.
#[allow(non_upper_case_globals)]
impl RegisterA64 {
    pub(crate) const KIND_MASK: u8 = 0x07;
    pub(crate) const INDEX_MASK: u8 = 0xF8;
    pub(crate) const INDEX_SHIFT: u32 = 3;

    const fn make(kind: KindA64, index: u8) -> Self {
        RegisterA64 {
            bits: (index << Self::INDEX_SHIFT) | (kind as u8),
        }
    }

    pub fn kind(&self) -> KindA64 {
        // C++ reads the 3-bit `kind` bitfield as a raw reinterpret. Rust forbids
        // an out-of-range enum value (a `transmute` of an invalid discriminant
        // is a NON-UNWINDING panic that aborts the whole process), so match
        // explicitly. Only 0..=5 are valid; an out-of-range encoding signals an
        // upstream bug — collapse it to `none`, which (like C++'s out-of-range
        // read) compares unequal to every real GPR/FP kind in the assert checks,
        // turning a process abort into a localized, debuggable test failure.
        match self.bits & Self::KIND_MASK {
            0 => KindA64::none,
            1 => KindA64::w,
            2 => KindA64::x,
            3 => KindA64::s,
            4 => KindA64::d,
            5 => KindA64::q,
            _ => KindA64::none,
        }
    }

    pub fn index(&self) -> u8 {
        (self.bits & Self::INDEX_MASK) >> Self::INDEX_SHIFT
    }

    // RegisterA64.h register constants.
    pub const noreg: RegisterA64 = Self::make(KindA64::none, 0);
    pub const w0: RegisterA64 = Self::make(KindA64::w, 0);
    pub const w1: RegisterA64 = Self::make(KindA64::w, 1);
    pub const w2: RegisterA64 = Self::make(KindA64::w, 2);
    pub const w3: RegisterA64 = Self::make(KindA64::w, 3);
    pub const w4: RegisterA64 = Self::make(KindA64::w, 4);
    pub const w5: RegisterA64 = Self::make(KindA64::w, 5);
    pub const w6: RegisterA64 = Self::make(KindA64::w, 6);
    pub const w7: RegisterA64 = Self::make(KindA64::w, 7);
    pub const w8: RegisterA64 = Self::make(KindA64::w, 8);
    pub const w9: RegisterA64 = Self::make(KindA64::w, 9);
    pub const w10: RegisterA64 = Self::make(KindA64::w, 10);
    pub const w11: RegisterA64 = Self::make(KindA64::w, 11);
    pub const w12: RegisterA64 = Self::make(KindA64::w, 12);
    pub const w13: RegisterA64 = Self::make(KindA64::w, 13);
    pub const w14: RegisterA64 = Self::make(KindA64::w, 14);
    pub const w15: RegisterA64 = Self::make(KindA64::w, 15);
    pub const w16: RegisterA64 = Self::make(KindA64::w, 16);
    pub const w17: RegisterA64 = Self::make(KindA64::w, 17);
    pub const w18: RegisterA64 = Self::make(KindA64::w, 18);
    pub const w19: RegisterA64 = Self::make(KindA64::w, 19);
    pub const w20: RegisterA64 = Self::make(KindA64::w, 20);
    pub const w21: RegisterA64 = Self::make(KindA64::w, 21);
    pub const w22: RegisterA64 = Self::make(KindA64::w, 22);
    pub const w23: RegisterA64 = Self::make(KindA64::w, 23);
    pub const w24: RegisterA64 = Self::make(KindA64::w, 24);
    pub const w25: RegisterA64 = Self::make(KindA64::w, 25);
    pub const w26: RegisterA64 = Self::make(KindA64::w, 26);
    pub const w27: RegisterA64 = Self::make(KindA64::w, 27);
    pub const w28: RegisterA64 = Self::make(KindA64::w, 28);
    pub const w29: RegisterA64 = Self::make(KindA64::w, 29);
    pub const w30: RegisterA64 = Self::make(KindA64::w, 30);
    pub const wzr: RegisterA64 = Self::make(KindA64::w, 31);
    pub const x0: RegisterA64 = Self::make(KindA64::x, 0);
    pub const x1: RegisterA64 = Self::make(KindA64::x, 1);
    pub const x2: RegisterA64 = Self::make(KindA64::x, 2);
    pub const x3: RegisterA64 = Self::make(KindA64::x, 3);
    pub const x4: RegisterA64 = Self::make(KindA64::x, 4);
    pub const x5: RegisterA64 = Self::make(KindA64::x, 5);
    pub const x6: RegisterA64 = Self::make(KindA64::x, 6);
    pub const x7: RegisterA64 = Self::make(KindA64::x, 7);
    pub const x8: RegisterA64 = Self::make(KindA64::x, 8);
    pub const x9: RegisterA64 = Self::make(KindA64::x, 9);
    pub const x10: RegisterA64 = Self::make(KindA64::x, 10);
    pub const x11: RegisterA64 = Self::make(KindA64::x, 11);
    pub const x12: RegisterA64 = Self::make(KindA64::x, 12);
    pub const x13: RegisterA64 = Self::make(KindA64::x, 13);
    pub const x14: RegisterA64 = Self::make(KindA64::x, 14);
    pub const x15: RegisterA64 = Self::make(KindA64::x, 15);
    pub const x16: RegisterA64 = Self::make(KindA64::x, 16);
    pub const x17: RegisterA64 = Self::make(KindA64::x, 17);
    pub const x18: RegisterA64 = Self::make(KindA64::x, 18);
    pub const x19: RegisterA64 = Self::make(KindA64::x, 19);
    pub const x20: RegisterA64 = Self::make(KindA64::x, 20);
    pub const x21: RegisterA64 = Self::make(KindA64::x, 21);
    pub const x22: RegisterA64 = Self::make(KindA64::x, 22);
    pub const x23: RegisterA64 = Self::make(KindA64::x, 23);
    pub const x24: RegisterA64 = Self::make(KindA64::x, 24);
    pub const x25: RegisterA64 = Self::make(KindA64::x, 25);
    pub const x26: RegisterA64 = Self::make(KindA64::x, 26);
    pub const x27: RegisterA64 = Self::make(KindA64::x, 27);
    pub const x28: RegisterA64 = Self::make(KindA64::x, 28);
    pub const x29: RegisterA64 = Self::make(KindA64::x, 29);
    pub const x30: RegisterA64 = Self::make(KindA64::x, 30);
    pub const xzr: RegisterA64 = Self::make(KindA64::x, 31);
    pub const sp: RegisterA64 = Self::make(KindA64::none, 31);
    pub const s0: RegisterA64 = Self::make(KindA64::s, 0);
    pub const s1: RegisterA64 = Self::make(KindA64::s, 1);
    pub const s2: RegisterA64 = Self::make(KindA64::s, 2);
    pub const s3: RegisterA64 = Self::make(KindA64::s, 3);
    pub const s4: RegisterA64 = Self::make(KindA64::s, 4);
    pub const s5: RegisterA64 = Self::make(KindA64::s, 5);
    pub const s6: RegisterA64 = Self::make(KindA64::s, 6);
    pub const s7: RegisterA64 = Self::make(KindA64::s, 7);
    pub const s8: RegisterA64 = Self::make(KindA64::s, 8);
    pub const s9: RegisterA64 = Self::make(KindA64::s, 9);
    pub const s10: RegisterA64 = Self::make(KindA64::s, 10);
    pub const s11: RegisterA64 = Self::make(KindA64::s, 11);
    pub const s12: RegisterA64 = Self::make(KindA64::s, 12);
    pub const s13: RegisterA64 = Self::make(KindA64::s, 13);
    pub const s14: RegisterA64 = Self::make(KindA64::s, 14);
    pub const s15: RegisterA64 = Self::make(KindA64::s, 15);
    pub const s16: RegisterA64 = Self::make(KindA64::s, 16);
    pub const s17: RegisterA64 = Self::make(KindA64::s, 17);
    pub const s18: RegisterA64 = Self::make(KindA64::s, 18);
    pub const s19: RegisterA64 = Self::make(KindA64::s, 19);
    pub const s20: RegisterA64 = Self::make(KindA64::s, 20);
    pub const s21: RegisterA64 = Self::make(KindA64::s, 21);
    pub const s22: RegisterA64 = Self::make(KindA64::s, 22);
    pub const s23: RegisterA64 = Self::make(KindA64::s, 23);
    pub const s24: RegisterA64 = Self::make(KindA64::s, 24);
    pub const s25: RegisterA64 = Self::make(KindA64::s, 25);
    pub const s26: RegisterA64 = Self::make(KindA64::s, 26);
    pub const s27: RegisterA64 = Self::make(KindA64::s, 27);
    pub const s28: RegisterA64 = Self::make(KindA64::s, 28);
    pub const s29: RegisterA64 = Self::make(KindA64::s, 29);
    pub const s30: RegisterA64 = Self::make(KindA64::s, 30);
    pub const s31: RegisterA64 = Self::make(KindA64::s, 31);
    pub const d0: RegisterA64 = Self::make(KindA64::d, 0);
    pub const d1: RegisterA64 = Self::make(KindA64::d, 1);
    pub const d2: RegisterA64 = Self::make(KindA64::d, 2);
    pub const d3: RegisterA64 = Self::make(KindA64::d, 3);
    pub const d4: RegisterA64 = Self::make(KindA64::d, 4);
    pub const d5: RegisterA64 = Self::make(KindA64::d, 5);
    pub const d6: RegisterA64 = Self::make(KindA64::d, 6);
    pub const d7: RegisterA64 = Self::make(KindA64::d, 7);
    pub const d8: RegisterA64 = Self::make(KindA64::d, 8);
    pub const d9: RegisterA64 = Self::make(KindA64::d, 9);
    pub const d10: RegisterA64 = Self::make(KindA64::d, 10);
    pub const d11: RegisterA64 = Self::make(KindA64::d, 11);
    pub const d12: RegisterA64 = Self::make(KindA64::d, 12);
    pub const d13: RegisterA64 = Self::make(KindA64::d, 13);
    pub const d14: RegisterA64 = Self::make(KindA64::d, 14);
    pub const d15: RegisterA64 = Self::make(KindA64::d, 15);
    pub const d16: RegisterA64 = Self::make(KindA64::d, 16);
    pub const d17: RegisterA64 = Self::make(KindA64::d, 17);
    pub const d18: RegisterA64 = Self::make(KindA64::d, 18);
    pub const d19: RegisterA64 = Self::make(KindA64::d, 19);
    pub const d20: RegisterA64 = Self::make(KindA64::d, 20);
    pub const d21: RegisterA64 = Self::make(KindA64::d, 21);
    pub const d22: RegisterA64 = Self::make(KindA64::d, 22);
    pub const d23: RegisterA64 = Self::make(KindA64::d, 23);
    pub const d24: RegisterA64 = Self::make(KindA64::d, 24);
    pub const d25: RegisterA64 = Self::make(KindA64::d, 25);
    pub const d26: RegisterA64 = Self::make(KindA64::d, 26);
    pub const d27: RegisterA64 = Self::make(KindA64::d, 27);
    pub const d28: RegisterA64 = Self::make(KindA64::d, 28);
    pub const d29: RegisterA64 = Self::make(KindA64::d, 29);
    pub const d30: RegisterA64 = Self::make(KindA64::d, 30);
    pub const d31: RegisterA64 = Self::make(KindA64::d, 31);
    pub const q0: RegisterA64 = Self::make(KindA64::q, 0);
    pub const q1: RegisterA64 = Self::make(KindA64::q, 1);
    pub const q2: RegisterA64 = Self::make(KindA64::q, 2);
    pub const q3: RegisterA64 = Self::make(KindA64::q, 3);
    pub const q4: RegisterA64 = Self::make(KindA64::q, 4);
    pub const q5: RegisterA64 = Self::make(KindA64::q, 5);
    pub const q6: RegisterA64 = Self::make(KindA64::q, 6);
    pub const q7: RegisterA64 = Self::make(KindA64::q, 7);
    pub const q8: RegisterA64 = Self::make(KindA64::q, 8);
    pub const q9: RegisterA64 = Self::make(KindA64::q, 9);
    pub const q10: RegisterA64 = Self::make(KindA64::q, 10);
    pub const q11: RegisterA64 = Self::make(KindA64::q, 11);
    pub const q12: RegisterA64 = Self::make(KindA64::q, 12);
    pub const q13: RegisterA64 = Self::make(KindA64::q, 13);
    pub const q14: RegisterA64 = Self::make(KindA64::q, 14);
    pub const q15: RegisterA64 = Self::make(KindA64::q, 15);
    pub const q16: RegisterA64 = Self::make(KindA64::q, 16);
    pub const q17: RegisterA64 = Self::make(KindA64::q, 17);
    pub const q18: RegisterA64 = Self::make(KindA64::q, 18);
    pub const q19: RegisterA64 = Self::make(KindA64::q, 19);
    pub const q20: RegisterA64 = Self::make(KindA64::q, 20);
    pub const q21: RegisterA64 = Self::make(KindA64::q, 21);
    pub const q22: RegisterA64 = Self::make(KindA64::q, 22);
    pub const q23: RegisterA64 = Self::make(KindA64::q, 23);
    pub const q24: RegisterA64 = Self::make(KindA64::q, 24);
    pub const q25: RegisterA64 = Self::make(KindA64::q, 25);
    pub const q26: RegisterA64 = Self::make(KindA64::q, 26);
    pub const q27: RegisterA64 = Self::make(KindA64::q, 27);
    pub const q28: RegisterA64 = Self::make(KindA64::q, 28);
    pub const q29: RegisterA64 = Self::make(KindA64::q, 29);
    pub const q30: RegisterA64 = Self::make(KindA64::q, 30);
    pub const q31: RegisterA64 = Self::make(KindA64::q, 31);
}

impl Default for RegisterA64 {
    fn default() -> Self {
        Self {
            bits: KindA64::none as u8,
        }
    }
}
