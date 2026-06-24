use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn fresh_type_pack(&mut self, scope: *mut Scope, polarity: Polarity) -> TypePackId {
        // FreeTypePack{scope, polarity}
        let mut free = FreeTypePack {
            index: 0,
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            polarity: Polarity::None,
        };
        free.free_type_pack_scope_polarity(scope, polarity);

        let allocated = self
            .type_packs
            .allocate(crate::records::type_pack_var::TypePackVar::from(free));
        unsafe {
            (*as_mutable_type_pack(allocated)).owningArena = self as *mut TypeArena;
        }
        allocated
    }
}
