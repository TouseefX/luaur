use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::register_x_64::RegisterX64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct OperandX64 {
    pub(crate) cat: CategoryX64,
    pub(crate) index: RegisterX64,
    pub(crate) base: RegisterX64,
    pub(crate) memSize: SizeX64,
    pub(crate) scale: u8,
    // Public in the C++ `OperandX64` struct; the constant-cache tests read `.imm`.
    pub imm: i32,
}

impl OperandX64 {
    pub const fn reg(reg: RegisterX64) -> Self {
        Self {
            cat: CategoryX64::reg,
            index: RegisterX64::noreg,
            base: reg,
            memSize: SizeX64::none,
            scale: 1,
            imm: 0,
        }
    }

    pub const fn imm(imm: i32) -> Self {
        Self {
            cat: CategoryX64::imm,
            // C++ uses `index(noreg), base(noreg)` — NOT register index 0. The
            // `{bits:0}` mistranslation made `qword[imm]` absolute addressing look
            // like it had base/index registers (wrong SIB/ModRM).
            index: RegisterX64::noreg,
            base: RegisterX64::noreg,
            memSize: SizeX64::none,
            scale: 1,
            imm,
        }
    }

    pub const fn mem(
        size: SizeX64,
        index: RegisterX64,
        scale: u8,
        base: RegisterX64,
        disp: i32,
    ) -> Self {
        Self {
            cat: CategoryX64::mem,
            index,
            base,
            memSize: size,
            scale,
            imm: disp,
        }
    }

    pub const fn operator_bracket(&self, mut address: OperandX64) -> OperandX64 {
        // CODEGEN_ASSERT(cat == CategoryX64::mem);
        // CODEGEN_ASSERT(index == noreg && scale == 1 && base == noreg && imm == 0);
        // CODEGEN_ASSERT(address.memSize == SizeX64::none);

        address.cat = CategoryX64::mem;
        address.memSize = self.memSize;
        address
    }
}

// OperandX64.h:75-81 — namespace-level size-prefix memory-operand globals. Used
// as `qword[rax]` in the assembler/tests: the `[]` is `operator_bracket`, which
// stamps this operand's `memSize` onto the address expression. They are C++
// `inline constexpr OperandX64 name{size, noreg, 1, noreg, 0}` (the `mem` ctor).
#[allow(non_upper_case_globals)]
pub const addr: OperandX64 =
    OperandX64::mem(SizeX64::none, RegisterX64::noreg, 1, RegisterX64::noreg, 0);
#[allow(non_upper_case_globals)]
pub const byte: OperandX64 =
    OperandX64::mem(SizeX64::byte, RegisterX64::noreg, 1, RegisterX64::noreg, 0);
#[allow(non_upper_case_globals)]
pub const word: OperandX64 =
    OperandX64::mem(SizeX64::word, RegisterX64::noreg, 1, RegisterX64::noreg, 0);
#[allow(non_upper_case_globals)]
pub const dword: OperandX64 =
    OperandX64::mem(SizeX64::dword, RegisterX64::noreg, 1, RegisterX64::noreg, 0);
#[allow(non_upper_case_globals)]
pub const qword: OperandX64 =
    OperandX64::mem(SizeX64::qword, RegisterX64::noreg, 1, RegisterX64::noreg, 0);
#[allow(non_upper_case_globals)]
pub const xmmword: OperandX64 = OperandX64::mem(
    SizeX64::xmmword,
    RegisterX64::noreg,
    1,
    RegisterX64::noreg,
    0,
);
#[allow(non_upper_case_globals)]
pub const ymmword: OperandX64 = OperandX64::mem(
    SizeX64::ymmword,
    RegisterX64::noreg,
    1,
    RegisterX64::noreg,
    0,
);

/// C++ implicit `OperandX64(RegisterX64 reg)` constructor.
impl From<RegisterX64> for OperandX64 {
    fn from(reg: RegisterX64) -> Self {
        Self::reg(reg)
    }
}

/// C++ implicit `OperandX64(int32_t imm)` constructor.
impl From<i32> for OperandX64 {
    fn from(imm: i32) -> Self {
        Self::imm(imm)
    }
}

// Ergonomic addressing-mode operators, delegating to the translated free
// functions so `qword[rax + r12 * 2 + 0x1b]`-style operands read like the C++
// (OperandX64.h `operator*`/`operator+`/`operator-`). The `[]` itself stays
// `operator_bracket` (Rust `Index` must return a reference, so it can't model
// the by-value size-stamping the assembler relies on).
impl core::ops::Mul<i32> for RegisterX64 {
    type Output = OperandX64;
    fn mul(self, scale: i32) -> OperandX64 {
        crate::functions::operator_deref::operator_deref(self, scale as u8)
    }
}

impl core::ops::Add<i32> for RegisterX64 {
    type Output = OperandX64;
    fn add(self, disp: i32) -> OperandX64 {
        crate::functions::operator_add_operand_x_64::operator_add_register_x_64_i32(self, disp)
    }
}

impl core::ops::Sub<i32> for RegisterX64 {
    type Output = OperandX64;
    fn sub(self, disp: i32) -> OperandX64 {
        crate::functions::operator_sub::operator_sub(self, disp)
    }
}

impl core::ops::Add<RegisterX64> for RegisterX64 {
    type Output = OperandX64;
    fn add(self, index: RegisterX64) -> OperandX64 {
        crate::functions::operator_add_operand_x_64_alt_b::operator_add_register_x_64_register_x_64(
            self, index,
        )
    }
}

impl core::ops::Add<OperandX64> for RegisterX64 {
    type Output = OperandX64;
    fn add(self, op: OperandX64) -> OperandX64 {
        crate::functions::operator_add_operand_x_64_alt_e::operator_add_register_x_64_operand_x_64(
            self, op,
        )
    }
}

impl core::ops::Add<i32> for OperandX64 {
    type Output = OperandX64;
    fn add(self, disp: i32) -> OperandX64 {
        crate::functions::operator_add_operand_x_64_alt_c::operator_add_operand_x_64_i32(self, disp)
    }
}

impl core::ops::Add<RegisterX64> for OperandX64 {
    type Output = OperandX64;
    fn add(self, base: RegisterX64) -> OperandX64 {
        crate::functions::operator_add_operand_x_64_alt_d::operator_add_operand_x_64_register_x_64(
            self, base,
        )
    }
}
