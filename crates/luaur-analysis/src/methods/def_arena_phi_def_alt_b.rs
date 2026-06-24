//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Def.cpp:49:DefArena::phi`
//! Source: `Analysis/src/Def.cpp` (Def.cpp:49-60, hand-ported)

use crate::functions::collect_operands::collect_operands;
use crate::records::def::Def;
use crate::records::def_arena::DefArena;
use crate::records::phi::Phi;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::variant::Variant as DefVariant;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

impl DefArena {
    pub fn phi_vector_def_id(&mut self, defs: &Vec<DefId>) -> DefId {
        let mut operands: Vec<DefId> = Vec::new();
        for &operand in defs.iter() {
            collect_operands(operand, &mut operands);
        }

        // There's no need to allocate a Phi node for a singleton set.
        if operands.len() == 1 {
            operands[0]
        } else {
            self.allocator.allocate(Def {
                v: DefVariant::V1(Phi { operands }),
                name: Symbol::default(),
                location: Location::default(),
            }) as DefId
        }
    }
}
