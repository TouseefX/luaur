use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;

const LOP_LOADB: u32 = 3;

pub fn is_direct_compare(
    proto: *const luaur_vm::records::proto::Proto,
    pc: *const Instruction,
    i: i32,
) -> bool {
    let proto = unsafe { &*proto };
    if i + 3 < proto.sizecode as i32 {
        let pc_val = unsafe { *pc };
        if LUAU_INSN_D(pc_val) == 2 {
            let load_true = unsafe { *pc.add(2) };
            let load_false = unsafe { *pc.add(3) };

            if LUAU_INSN_OP(load_true) == LOP_LOADB && LUAU_INSN_OP(load_false) == LOP_LOADB {
                let same_target = LUAU_INSN_A(load_true) == LUAU_INSN_A(load_false);
                let zero_and_one = LUAU_INSN_B(load_true) == 0 && LUAU_INSN_B(load_false) == 1;
                let correct_jumps = LUAU_INSN_C(load_true) == 1 && LUAU_INSN_C(load_false) == 0;

                return same_target && zero_and_one && correct_jumps;
            }
        }
    }

    false
}
