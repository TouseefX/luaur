use crate::functions::unfreeze::unfreeze;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use luaur_common::FFlag;

impl Drop for BuiltinTypes {
    fn drop(&mut self) {
        let previous = FFlag::DebugLuauFreezeArena.get_global();
        FFlag::DebugLuauFreezeArena.set(self.debugFreezeArena);

        unfreeze(&mut self.arena);
        let arena = core::mem::replace(
            &mut self.arena,
            alloc::boxed::Box::new(TypeArena::default()),
        );
        drop(arena);

        FFlag::DebugLuauFreezeArena.set(previous);
    }
}
