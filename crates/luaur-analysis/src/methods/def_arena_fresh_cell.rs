//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Def.cpp:39:DefArena::freshCell`
//! Source: `Analysis/src/Def.cpp` (Def.cpp:39-42, hand-ported)

use crate::records::cell::Cell;
use crate::records::def::Def;
use crate::records::def_arena::DefArena;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::variant::Variant as DefVariant;
use luaur_ast::records::location::Location;

impl DefArena {
    pub fn fresh_cell(&mut self, sym: Symbol, location: Location, subscripted: bool) -> DefId {
        // NotNull{allocator.allocate(Def{Cell{subscripted}, sym, location})}
        self.allocator.allocate(Def {
            v: DefVariant::V0(Cell { subscripted }),
            name: sym,
            location,
        }) as DefId
    }
}
