use crate::records::type_arena::TypeArena;
use luaur_common::FFlag;

pub fn unfreeze(arena: &mut TypeArena) {
    if !FFlag::DebugLuauFreezeArena.get() {
        return;
    }

    arena.types.unfreeze();
    arena.type_packs.unfreeze();
}
