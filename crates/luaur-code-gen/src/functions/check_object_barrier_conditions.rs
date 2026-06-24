use crate::enums::condition_x_64::ConditionX64;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::size_x_64::SizeX64;
use crate::functions::dword_reg::dword_reg;
use crate::functions::is_gco::is_gco;
use crate::functions::luau_constant_tag::luau_constant_tag;
use crate::functions::luau_constant_value::luau_constant_value;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::functions::vm_const_op::vm_const_op;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::bitmask::{bit2mask, bitmask};
use luaur_vm::macros::blackbit::BLACKBIT;
use luaur_vm::macros::white_0_bit::WHITE0BIT;
use luaur_vm::macros::white_1_bit::WHITE1BIT;
use luaur_vm::records::g_cheader::GCheader;

pub fn check_object_barrier_conditions(
    build: &mut AssemblyBuilderX64,
    tmp: RegisterX64,
    object: RegisterX64,
    ra: RegisterX64,
    ra_op: IrOp,
    ratag: i32,
    skip: &mut Label,
) {
    // Barrier should've been optimized away if we know that it's not collectable, checking for correctness
    if ratag == -1 || !is_gco(ratag as u8) {
        // iscollectable(ra)
        if ra_op.kind() == IrOpKind::Inst {
            build.vpextrd(dword_reg(tmp), ra, 3);
            build.cmp(
                OperandX64::reg(dword_reg(tmp)),
                OperandX64::imm(lua_Type::LUA_TSTRING as i32),
            );
        } else {
            let tag = if ra_op.kind() == IrOpKind::VmReg {
                luau_reg_tag(vm_reg_op(ra_op))
            } else {
                luau_constant_tag(vm_const_op(ra_op))
            };
            build.cmp(tag, OperandX64::imm(lua_Type::LUA_TSTRING as i32));
        }

        build.jcc(ConditionX64::Less, skip);
    }

    // isblack(obj2gco(o))
    build.test(
        OperandX64::mem(
            SizeX64::byte,
            RegisterX64::noreg,
            1,
            object,
            core::mem::offset_of!(GCheader, marked) as i32,
        ),
        OperandX64::imm(bitmask(BLACKBIT as i32)),
    );
    build.jcc(ConditionX64::Zero, skip);

    // iswhite(gcvalue(ra))
    if ra_op.kind() == IrOpKind::Inst {
        build.vmovq(OperandX64::reg(tmp), OperandX64::reg(ra));
    } else {
        let value = if ra_op.kind() == IrOpKind::VmReg {
            luau_reg_value(vm_reg_op(ra_op))
        } else {
            luau_constant_value(vm_const_op(ra_op))
        };
        build.mov(OperandX64::reg(tmp), value);
    }
    build.test(
        OperandX64::mem(
            SizeX64::byte,
            RegisterX64::noreg,
            1,
            tmp,
            core::mem::offset_of!(GCheader, marked) as i32,
        ),
        OperandX64::imm(bit2mask(WHITE0BIT, WHITE1BIT as i32)),
    );
    build.jcc(ConditionX64::Zero, skip);
}
