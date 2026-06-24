//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Instantiation.cpp:197:instantiate`
//! Source: `Analysis/src/Instantiation.cpp` (Instantiation.cpp:197-257, hand-ported)

use crate::enums::polarity::Polarity;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::clone_state::CloneState;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::replacer::Replacer;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub fn instantiate(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    limits: *mut TypeCheckLimits,
    scope: *mut Scope,
    ty: TypeId,
) -> Option<TypeId> {
    unsafe {
        // ty = follow(ty);
        let ty = follow_type_id(ty);

        // const FunctionType* ft = get<FunctionType>(ty);
        // if (!ft) return ty;
        let ft = get_type_id::<FunctionType>(ty);
        if ft.is_null() {
            return Some(ty);
        }
        let ft = &*ft;

        // if (ft->generics.empty() && ft->genericPacks.empty()) return ty;
        if ft.generics.is_empty() && ft.generic_packs.is_empty() {
            return Some(ty);
        }

        // DenseHashMap<TypeId, TypeId> replacements{nullptr};
        // DenseHashMap<TypePackId, TypePackId> replacementPacks{nullptr};
        let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::new(core::ptr::null());
        let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> =
            DenseHashMap::new(core::ptr::null());

        if crate::FFlag::LuauInstantiationUsesPolarity.get() {
            // for (TypeId g : ft->generics)
            //     if (auto gen = get<GenericType>(follow(g)))
            //         replacements[g] = freshType(arena, builtinTypes, scope, gen->polarity);
            for &g in &ft.generics {
                let gen = get_type_id::<GenericType>(follow_type_id(g));
                if !gen.is_null() {
                    *replacements.get_or_insert(g) =
                        fresh_type(&mut *arena, &*builtin_types, scope, (*gen).polarity);
                }
            }

            // for (TypePackId g : ft->genericPacks)
            //     if (auto gen = get<GenericTypePack>(follow(g)))
            //         replacementPacks[g] = arena->freshTypePack(scope, gen->polarity);
            for &g in &ft.generic_packs {
                let gen = get_type_pack_id::<GenericTypePack>(follow_type_pack_id(g));
                if !gen.is_null() {
                    *replacement_packs.get_or_insert(g) =
                        (*arena).fresh_type_pack(scope, (*gen).polarity);
                }
            }
        } else {
            // for (TypeId g : ft->generics)
            //     replacements[g] = freshType(arena, builtinTypes, scope);
            for &g in &ft.generics {
                *replacements.get_or_insert(g) =
                    fresh_type(&mut *arena, &*builtin_types, scope, Polarity::None);
            }

            // for (TypePackId g : ft->genericPacks)
            //     replacementPacks[g] = arena->freshTypePack(scope);
            for &g in &ft.generic_packs {
                *replacement_packs.get_or_insert(g) =
                    (*arena).fresh_type_pack(scope, Polarity::None);
            }
        }

        // Replacer r{arena, NotNull{&replacements}, NotNull{&replacementPacks}};
        let mut r = Replacer::replacer(
            arena,
            &mut replacements as *mut _,
            &mut replacement_packs as *mut _,
        );

        // if (limits->instantiationChildLimit)
        //     r.childLimit = *limits->instantiationChildLimit;
        if let Some(child_limit) = (*limits).instantiationChildLimit {
            r.base.base.child_limit = child_limit;
        }

        // CloneState cs{builtinTypes};
        let mut cs = CloneState {
            builtin_types,
            seen_types: DenseHashMap::new(core::ptr::null()),
            seen_type_packs: DenseHashMap::new(core::ptr::null()),
        };

        // auto clonedFunctionTypeId = shallowClone(ty, *arena, cs, /* clonePersistentTypes */ true);
        let cloned_function_type_id = shallow_clone(ty, &mut *arena, &mut cs, true);

        // FunctionType* ft2 = getMutable<FunctionType>(clonedFunctionTypeId);
        let ft2 = get_mutable_type_id::<FunctionType>(cloned_function_type_id);

        // ft2->generics.clear();
        // ft2->genericPacks.clear();
        (*ft2).generics.clear();
        (*ft2).generic_packs.clear();

        // return r.substitute(clonedFunctionTypeId);
        r.substitute_type_id(cloned_function_type_id)
    }
}
