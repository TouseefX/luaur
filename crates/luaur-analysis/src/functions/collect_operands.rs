use crate::functions::get_def::get_def_id;
use crate::records::cell::Cell;
use crate::records::phi::Phi;
use crate::type_aliases::def_id_def::DefId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn collect_operands(def: DefId, operands: &mut Vec<DefId>) {
    unsafe {
        LUAU_ASSERT!(operands as *const Vec<DefId> as *const () as usize != 0);

        if operands.iter().any(|&d| d == def) {
            return;
        }

        if !get_def_id::<Cell>(def).is_null() {
            operands.push(def);
        } else if !get_def_id::<Phi>(def).is_null() {
            let phi = get_def_id::<Phi>(def);
            if (*phi).operands.is_empty() {
                operands.push(def);
            } else {
                for &operand in (*phi).operands.iter() {
                    collect_operands(operand, operands);
                }
            }
        }
    }
}
