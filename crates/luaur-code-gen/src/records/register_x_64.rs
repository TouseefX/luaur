use crate::enums::size_x_64::SizeX64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RegisterX64 {
    pub(crate) bits: u8,
}

// The C++ `RegisterX64` is `{SizeX64 size; uint8_t index;}`; this Rust port packs
// both into one `bits` byte (size in the low 3 bits, index in the high 5), so the
// `inline constexpr RegisterX64 name{size, index}` globals from RegisterX64.h
// become associated consts. `make` reproduces that constructor exactly.
#[allow(non_upper_case_globals)]
impl RegisterX64 {
    pub(crate) const SIZE_MASK: u8 = 0x07;
    pub(crate) const INDEX_MASK: u8 = 0xF8;
    pub(crate) const INDEX_SHIFT: u32 = 3;

    /// C++ `RegisterX64{size, index}` — pack `{size, index}` into the bit layout.
    const fn make(size: SizeX64, index: u8) -> Self {
        RegisterX64 {
            bits: (index << Self::INDEX_SHIFT) | (size as u8),
        }
    }

    pub fn size(&self) -> SizeX64 {
        unsafe { core::mem::transmute(self.bits & Self::SIZE_MASK) }
    }

    pub const fn index(&self) -> u8 {
        (self.bits & Self::INDEX_MASK) >> Self::INDEX_SHIFT
    }

    // RegisterX64.h:42-43 — sentinels.
    pub const noreg: RegisterX64 = Self::make(SizeX64::none, 16);
    pub const rip: RegisterX64 = Self::make(SizeX64::none, 0);

    // RegisterX64.h:45-60 — byte registers.
    pub const al: RegisterX64 = Self::make(SizeX64::byte, 0);
    pub const cl: RegisterX64 = Self::make(SizeX64::byte, 1);
    pub const dl: RegisterX64 = Self::make(SizeX64::byte, 2);
    pub const bl: RegisterX64 = Self::make(SizeX64::byte, 3);
    pub const spl: RegisterX64 = Self::make(SizeX64::byte, 4);
    pub const bpl: RegisterX64 = Self::make(SizeX64::byte, 5);
    pub const sil: RegisterX64 = Self::make(SizeX64::byte, 6);
    pub const dil: RegisterX64 = Self::make(SizeX64::byte, 7);
    pub const r8b: RegisterX64 = Self::make(SizeX64::byte, 8);
    pub const r9b: RegisterX64 = Self::make(SizeX64::byte, 9);
    pub const r10b: RegisterX64 = Self::make(SizeX64::byte, 10);
    pub const r11b: RegisterX64 = Self::make(SizeX64::byte, 11);
    pub const r12b: RegisterX64 = Self::make(SizeX64::byte, 12);
    pub const r13b: RegisterX64 = Self::make(SizeX64::byte, 13);
    pub const r14b: RegisterX64 = Self::make(SizeX64::byte, 14);
    pub const r15b: RegisterX64 = Self::make(SizeX64::byte, 15);

    // RegisterX64.h:62-77 — dword registers.
    pub const eax: RegisterX64 = Self::make(SizeX64::dword, 0);
    pub const ecx: RegisterX64 = Self::make(SizeX64::dword, 1);
    pub const edx: RegisterX64 = Self::make(SizeX64::dword, 2);
    pub const ebx: RegisterX64 = Self::make(SizeX64::dword, 3);
    pub const esp: RegisterX64 = Self::make(SizeX64::dword, 4);
    pub const ebp: RegisterX64 = Self::make(SizeX64::dword, 5);
    pub const esi: RegisterX64 = Self::make(SizeX64::dword, 6);
    pub const edi: RegisterX64 = Self::make(SizeX64::dword, 7);
    pub const r8d: RegisterX64 = Self::make(SizeX64::dword, 8);
    pub const r9d: RegisterX64 = Self::make(SizeX64::dword, 9);
    pub const r10d: RegisterX64 = Self::make(SizeX64::dword, 10);
    pub const r11d: RegisterX64 = Self::make(SizeX64::dword, 11);
    pub const r12d: RegisterX64 = Self::make(SizeX64::dword, 12);
    pub const r13d: RegisterX64 = Self::make(SizeX64::dword, 13);
    pub const r14d: RegisterX64 = Self::make(SizeX64::dword, 14);
    pub const r15d: RegisterX64 = Self::make(SizeX64::dword, 15);

    // RegisterX64.h:79-94 — qword registers.
    pub const rax: RegisterX64 = Self::make(SizeX64::qword, 0);
    pub const rcx: RegisterX64 = Self::make(SizeX64::qword, 1);
    pub const rdx: RegisterX64 = Self::make(SizeX64::qword, 2);
    pub const rbx: RegisterX64 = Self::make(SizeX64::qword, 3);
    pub const rsp: RegisterX64 = Self::make(SizeX64::qword, 4);
    pub const rbp: RegisterX64 = Self::make(SizeX64::qword, 5);
    pub const rsi: RegisterX64 = Self::make(SizeX64::qword, 6);
    pub const rdi: RegisterX64 = Self::make(SizeX64::qword, 7);
    pub const r8: RegisterX64 = Self::make(SizeX64::qword, 8);
    pub const r9: RegisterX64 = Self::make(SizeX64::qword, 9);
    pub const r10: RegisterX64 = Self::make(SizeX64::qword, 10);
    pub const r11: RegisterX64 = Self::make(SizeX64::qword, 11);
    pub const r12: RegisterX64 = Self::make(SizeX64::qword, 12);
    pub const r13: RegisterX64 = Self::make(SizeX64::qword, 13);
    pub const r14: RegisterX64 = Self::make(SizeX64::qword, 14);
    pub const r15: RegisterX64 = Self::make(SizeX64::qword, 15);

    // RegisterX64.h:96-111 — xmm registers.
    pub const xmm0: RegisterX64 = Self::make(SizeX64::xmmword, 0);
    pub const xmm1: RegisterX64 = Self::make(SizeX64::xmmword, 1);
    pub const xmm2: RegisterX64 = Self::make(SizeX64::xmmword, 2);
    pub const xmm3: RegisterX64 = Self::make(SizeX64::xmmword, 3);
    pub const xmm4: RegisterX64 = Self::make(SizeX64::xmmword, 4);
    pub const xmm5: RegisterX64 = Self::make(SizeX64::xmmword, 5);
    pub const xmm6: RegisterX64 = Self::make(SizeX64::xmmword, 6);
    pub const xmm7: RegisterX64 = Self::make(SizeX64::xmmword, 7);
    pub const xmm8: RegisterX64 = Self::make(SizeX64::xmmword, 8);
    pub const xmm9: RegisterX64 = Self::make(SizeX64::xmmword, 9);
    pub const xmm10: RegisterX64 = Self::make(SizeX64::xmmword, 10);
    pub const xmm11: RegisterX64 = Self::make(SizeX64::xmmword, 11);
    pub const xmm12: RegisterX64 = Self::make(SizeX64::xmmword, 12);
    pub const xmm13: RegisterX64 = Self::make(SizeX64::xmmword, 13);
    pub const xmm14: RegisterX64 = Self::make(SizeX64::xmmword, 14);
    pub const xmm15: RegisterX64 = Self::make(SizeX64::xmmword, 15);

    // RegisterX64.h:113-128 — ymm registers.
    pub const ymm0: RegisterX64 = Self::make(SizeX64::ymmword, 0);
    pub const ymm1: RegisterX64 = Self::make(SizeX64::ymmword, 1);
    pub const ymm2: RegisterX64 = Self::make(SizeX64::ymmword, 2);
    pub const ymm3: RegisterX64 = Self::make(SizeX64::ymmword, 3);
    pub const ymm4: RegisterX64 = Self::make(SizeX64::ymmword, 4);
    pub const ymm5: RegisterX64 = Self::make(SizeX64::ymmword, 5);
    pub const ymm6: RegisterX64 = Self::make(SizeX64::ymmword, 6);
    pub const ymm7: RegisterX64 = Self::make(SizeX64::ymmword, 7);
    pub const ymm8: RegisterX64 = Self::make(SizeX64::ymmword, 8);
    pub const ymm9: RegisterX64 = Self::make(SizeX64::ymmword, 9);
    pub const ymm10: RegisterX64 = Self::make(SizeX64::ymmword, 10);
    pub const ymm11: RegisterX64 = Self::make(SizeX64::ymmword, 11);
    pub const ymm12: RegisterX64 = Self::make(SizeX64::ymmword, 12);
    pub const ymm13: RegisterX64 = Self::make(SizeX64::ymmword, 13);
    pub const ymm14: RegisterX64 = Self::make(SizeX64::ymmword, 14);
    pub const ymm15: RegisterX64 = Self::make(SizeX64::ymmword, 15);
}

impl Default for RegisterX64 {
    fn default() -> Self {
        Self {
            bits: SizeX64::none as u8,
        }
    }
}
