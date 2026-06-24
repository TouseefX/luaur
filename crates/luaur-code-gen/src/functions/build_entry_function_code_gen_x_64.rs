use crate::enums::abix_64::ABIX64;
use crate::enums::alignment_data_x_64::AlignmentDataX64;
use crate::enums::size_x_64::SizeX64;
use crate::functions::get_full_stack_size::{
    get_full_stack_size, kStackAlign, kStackOffsetToLocals,
};
use crate::functions::get_non_vol_xmm_storage_size::get_non_vol_xmm_storage_size;
use crate::functions::get_xmm_register_count::get_xmm_register_count;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::entry_locations_code_gen_x_64::EntryLocations;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::label::Label;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::unwind_builder::UnwindBuilder;
use alloc::vec::Vec;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::proto::Proto;
use luaur_vm::type_aliases::t_value::TValue;
use luaur_vm::type_aliases::value::Value;

const K_FUNCTION_ALIGNMENT: u32 = 32;
const K_WINDOWS_FIRST_NON_VOL_XMM_REG: u8 = 6;
const K_FULL_BLOCK_FUNCTION: u32 = 0xffff_ffff;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
    RegisterX64 {
        bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
    }
}

const R12: RegisterX64 = reg(12, SizeX64::qword);
const R13: RegisterX64 = reg(13, SizeX64::qword);
const R14: RegisterX64 = reg(14, SizeX64::qword);
const R15: RegisterX64 = reg(15, SizeX64::qword);

const R_CONSTANTS: RegisterX64 = R12;
const R_NATIVE_CONTEXT: RegisterX64 = R13;
const R_BASE: RegisterX64 = R14;
const R_STATE: RegisterX64 = R15;

fn mem(size: SizeX64, base: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(size, RegisterX64::noreg, 1, base, disp)
}

fn s_closure() -> OperandX64 {
    mem(
        SizeX64::qword,
        RegisterX64::rsp,
        kStackOffsetToLocals as i32,
    )
}

fn s_code() -> OperandX64 {
    mem(
        SizeX64::qword,
        RegisterX64::rsp,
        kStackOffsetToLocals as i32 + 8,
    )
}

fn set_fresh_label(build: &mut AssemblyBuilderX64) -> Label {
    let mut label = Label::default();
    build.set_label(&mut label);
    label
}

#[cfg(target_os = "windows")]
fn unwind_start_function(unwind: &mut UnwindBuilder) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
            .start_function();
    }
}

#[cfg(not(target_os = "windows"))]
fn unwind_start_function(unwind: &mut UnwindBuilder) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2>())
            .start_function();
    }
}

#[cfg(target_os = "windows")]
fn unwind_prologue_x_64(
    unwind: &mut UnwindBuilder,
    prologue_size: u32,
    full_stack_size: u32,
    setup_frame: bool,
    gpr: &[RegisterX64],
    simd: &[RegisterX64],
) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
            .prologue_x_64(prologue_size, full_stack_size, setup_frame, gpr, simd);
    }
}

#[cfg(not(target_os = "windows"))]
fn unwind_prologue_x_64(
    unwind: &mut UnwindBuilder,
    prologue_size: u32,
    full_stack_size: u32,
    setup_frame: bool,
    gpr: &[RegisterX64],
    simd: &[RegisterX64],
) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2>())
            .prologue_x_64(prologue_size, full_stack_size, setup_frame, gpr, simd);
    }
}

#[cfg(target_os = "windows")]
fn unwind_finish_function(unwind: &mut UnwindBuilder, begin_offset: u32, end_offset: u32) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_win::UnwindBuilderWin>())
            .finish_function(begin_offset, end_offset);
    }
}

#[cfg(not(target_os = "windows"))]
fn unwind_finish_function(unwind: &mut UnwindBuilder, begin_offset: u32, end_offset: u32) {
    unsafe {
        (&mut *(unwind as *mut UnwindBuilder)
            .cast::<crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2>())
            .finish_function(begin_offset, end_offset);
    }
}

pub fn build_entry_function(
    build: &mut AssemblyBuilderX64,
    unwind: &mut UnwindBuilder,
) -> EntryLocations {
    let mut locations = EntryLocations::default();

    build.align(K_FUNCTION_ALIGNMENT, AlignmentDataX64::Ud2);

    locations.start = set_fresh_label(build);
    unwind_start_function(unwind);

    let (r_arg1, r_arg2, r_arg3, r_arg4) =
        if luaur_common::FFlag::LuauCodegenSuggestArgumentRegisterX64.get() {
            (
                IrCallWrapperX64::suggest_argument_register::<0>(SizeX64::qword, build),
                IrCallWrapperX64::suggest_argument_register::<1>(SizeX64::qword, build),
                IrCallWrapperX64::suggest_argument_register::<2>(SizeX64::qword, build),
                IrCallWrapperX64::suggest_argument_register::<3>(SizeX64::qword, build),
            )
        } else if build.abi == ABIX64::Windows {
            (
                RegisterX64::rcx,
                RegisterX64::rdx,
                RegisterX64::r8,
                RegisterX64::r9,
            )
        } else {
            (
                RegisterX64::rdi,
                RegisterX64::rsi,
                RegisterX64::rdx,
                RegisterX64::rcx,
            )
        };

    if build.abi == ABIX64::SystemV {
        build.push(OperandX64::reg(RegisterX64::rbp));
        build.mov(
            OperandX64::reg(RegisterX64::rbp),
            OperandX64::reg(RegisterX64::rsp),
        );
    }

    build.push(OperandX64::reg(RegisterX64::rbx));
    build.push(OperandX64::reg(R12));
    build.push(OperandX64::reg(R13));
    build.push(OperandX64::reg(R14));
    build.push(OperandX64::reg(R15));

    if build.abi == ABIX64::Windows {
        build.push(OperandX64::reg(RegisterX64::rdi));
        build.push(OperandX64::reg(RegisterX64::rsi));
        build.push(OperandX64::reg(RegisterX64::rbp));
    }

    let usable_xmm_reg_count = get_xmm_register_count(build.abi);
    let xmm_storage_size = get_non_vol_xmm_storage_size(build.abi, usable_xmm_reg_count);
    let full_stack_size = get_full_stack_size(build.abi, usable_xmm_reg_count);

    build.sub(
        OperandX64::reg(RegisterX64::rsp),
        OperandX64::imm(full_stack_size as i32),
    );

    let xmm_storage_offset = full_stack_size as i32 - (kStackAlign + xmm_storage_size) as i32;
    let mut saved_xmm_regs: Vec<RegisterX64> = Vec::new();

    if build.abi == ABIX64::Windows {
        if usable_xmm_reg_count > K_WINDOWS_FIRST_NON_VOL_XMM_REG {
            saved_xmm_regs
                .reserve((usable_xmm_reg_count - K_WINDOWS_FIRST_NON_VOL_XMM_REG) as usize);
        }

        let mut offset = 0;
        for i in K_WINDOWS_FIRST_NON_VOL_XMM_REG..usable_xmm_reg_count {
            let xmm_reg = reg(i, SizeX64::xmmword);
            build.vmovaps(
                mem(
                    SizeX64::xmmword,
                    RegisterX64::rsp,
                    xmm_storage_offset + offset,
                ),
                OperandX64::reg(xmm_reg),
            );
            saved_xmm_regs.push(xmm_reg);
            offset += 16;
        }
    }

    locations.prologueEnd = set_fresh_label(build);

    let prologue_size =
        build.get_label_offset(&locations.prologueEnd) - build.get_label_offset(&locations.start);

    if build.abi == ABIX64::SystemV {
        unwind_prologue_x_64(
            unwind,
            prologue_size,
            full_stack_size,
            true,
            &[RegisterX64::rbx, R12, R13, R14, R15],
            &[],
        );
    } else if build.abi == ABIX64::Windows {
        unwind_prologue_x_64(
            unwind,
            prologue_size,
            full_stack_size,
            false,
            &[
                RegisterX64::rbx,
                R12,
                R13,
                R14,
                R15,
                RegisterX64::rdi,
                RegisterX64::rsi,
                RegisterX64::rbp,
            ],
            &saved_xmm_regs,
        );
    }

    build.mov(OperandX64::reg(R_STATE), OperandX64::reg(r_arg1));
    build.mov(OperandX64::reg(R_NATIVE_CONTEXT), OperandX64::reg(r_arg4));
    build.mov(
        OperandX64::reg(R_BASE),
        mem(
            SizeX64::qword,
            R_STATE,
            core::mem::offset_of!(lua_State, base) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            R_STATE,
            core::mem::offset_of!(lua_State, ci) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            RegisterX64::rax,
            core::mem::offset_of!(CallInfo, func) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            RegisterX64::rax,
            (core::mem::offset_of!(TValue, value) + core::mem::offset_of!(Value, gc)) as i32,
        ),
    );
    build.mov(s_closure(), OperandX64::reg(RegisterX64::rax));
    build.mov(
        OperandX64::reg(R_CONSTANTS),
        mem(
            SizeX64::qword,
            r_arg2,
            core::mem::offset_of!(Proto, k) as i32,
        ),
    );
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        mem(
            SizeX64::qword,
            r_arg2,
            core::mem::offset_of!(Proto, code) as i32,
        ),
    );
    build.mov(s_code(), OperandX64::reg(RegisterX64::rax));

    build.jmp_operand_x_64(OperandX64::reg(r_arg3));

    locations.epilogueStart = set_fresh_label(build);

    if build.abi == ABIX64::Windows {
        let mut offset = 0;
        for i in K_WINDOWS_FIRST_NON_VOL_XMM_REG..usable_xmm_reg_count {
            build.vmovaps(
                OperandX64::reg(reg(i, SizeX64::xmmword)),
                mem(
                    SizeX64::xmmword,
                    RegisterX64::rsp,
                    xmm_storage_offset + offset,
                ),
            );
            offset += 16;
        }
    }

    build.add(
        OperandX64::reg(RegisterX64::rsp),
        OperandX64::imm(full_stack_size as i32),
    );

    if build.abi == ABIX64::Windows {
        build.pop(OperandX64::reg(RegisterX64::rbp));
        build.pop(OperandX64::reg(RegisterX64::rsi));
        build.pop(OperandX64::reg(RegisterX64::rdi));
    }

    build.pop(OperandX64::reg(R15));
    build.pop(OperandX64::reg(R14));
    build.pop(OperandX64::reg(R13));
    build.pop(OperandX64::reg(R12));
    build.pop(OperandX64::reg(RegisterX64::rbx));

    if build.abi == ABIX64::SystemV {
        build.pop(OperandX64::reg(RegisterX64::rbp));
    }

    build.ret();

    unwind_finish_function(
        unwind,
        build.get_label_offset(&locations.start),
        K_FULL_BLOCK_FUNCTION,
    );

    locations
}
