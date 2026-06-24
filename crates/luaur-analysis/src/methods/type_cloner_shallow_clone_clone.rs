use crate::enums::follow_option::FollowOption;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type_alt_c::follow_type_id_follow_option;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::free_type::FreeType;
use crate::records::generic_type::GenericType;
use crate::records::r#type::Type;
use crate::records::table_type::TableType;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;

impl TypeCloner {
    pub fn shallow_clone_type_id(&mut self, ty: TypeId) -> TypeId {
        // We want to [`Luau::follow`] but without forcing the expansion of [`LazyType`]s.
        let ty = unsafe { follow_type_id_follow_option(ty, FollowOption::DisableLazyTypeThunks) };

        if let Some(clone) = self.find_type_id(ty) {
            return clone;
        } else if unsafe { (*ty).persistent } && ty != self.force_ty {
            return ty;
        }

        let target = unsafe { (*self.arena).add_type(Type::new((*ty).ty.clone())) };
        unsafe {
            (*as_mutable_type_id(target)).documentation_symbol = (*ty).documentation_symbol.clone();
        }

        // `replacement_for_null_scope` is null for ordinary clones (Clone.cpp:171-176,
        // free/table scope -> null) and carries the fragment cloner's fresh scope for
        // the `FragmentAutocompleteTypeCloner` override (Clone.cpp:508-513). Generic
        // types always get a null scope in both paths.
        unsafe {
            let generic = get_mutable_type_id::<GenericType>(target);
            if !generic.is_null() {
                (*generic).scope = core::ptr::null_mut();
            } else {
                let free = get_mutable_type_id::<FreeType>(target);
                if !free.is_null() {
                    (*free).scope = self.replacement_for_null_scope;
                } else {
                    let table = get_mutable_type_id::<TableType>(target);
                    if !table.is_null() {
                        (*table).scope = self.replacement_for_null_scope;
                    }
                }
            }
        }

        unsafe { (*self.types).insert(ty, target) };
        self.queue.push(TypeOrPack::V0(target));
        target
    }
}
