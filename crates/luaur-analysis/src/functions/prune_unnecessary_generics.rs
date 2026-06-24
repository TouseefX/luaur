use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_tail::get_tail;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::invert_polarity::invert;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::counter_state::CounterState;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FInt;
use std::collections::{HashMap, HashSet};

pub fn prune_unnecessary_generics(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    _scope: *mut Scope,
    cached_types: *mut DenseHashSet<TypeId>,
    ty: TypeId,
) {
    let ty = unsafe { follow_type_id(ty) };

    if unsafe { (*ty).owning_arena != arena || (*ty).persistent } {
        return;
    }

    let function_ty = unsafe { get_mutable_type_id::<FunctionType>(ty) };
    if function_ty.is_null() {
        return;
    }

    let mut counter = GenericCounterLocal::new(cached_types);

    unsafe {
        for generic in (*function_ty).generics.iter().copied() {
            let generic = follow_type_id(generic);
            let g = get_type_id::<GenericType>(generic);
            if !g.is_null() && !(*g).explicit_name {
                counter.generics.entry(generic).or_default();
            }
        }

        let mut i = 0;
        while i < (*function_ty).generic_packs.len() {
            let generic_pack = follow_type_pack_id((&(*function_ty).generic_packs)[i]);
            let tail = get_tail(generic_pack);

            if tail != generic_pack {
                (*function_ty).generic_packs.push(tail);
            }

            let g = get_type_pack_id::<GenericTypePack>(tail);
            if !g.is_null() && !(*g).explicitName {
                counter.generic_packs.entry(generic_pack).or_default();
            }

            i += 1;
        }
    }

    counter.traverse_type_id(ty);

    unsafe {
        if !counter.hit_limits {
            for (&generic, state) in counter.generics.iter() {
                if state.count == 1 && state.polarity != Polarity::Mixed {
                    if (*generic).owning_arena != arena {
                        continue;
                    }

                    (*as_mutable_type_id(generic)).ty =
                        TypeVariant::Bound((*builtin_types).unknownType);
                }
            }
        }

        let mut seen = HashSet::new();
        (*function_ty).generics.retain(|generic| {
            let generic = follow_type_id(*generic);
            if !seen.insert(generic) {
                return false;
            }

            if !counter.hit_limits {
                if let Some(state) = counter.generics.get(&generic) {
                    if state.count == 0 {
                        return false;
                    }
                }
            }

            !get_type_id::<GenericType>(generic).is_null()
        });

        if !counter.hit_limits {
            for (&generic_pack, state) in counter.generic_packs.iter() {
                if state.count == 1 {
                    (*as_mutable_type_pack_id(generic_pack)).ty =
                        TypePackVariant::Bound((*builtin_types).unknownTypePack);
                }
            }
        }

        let mut seen = HashSet::new();
        (*function_ty).generic_packs.retain(|generic_pack| {
            let generic_pack = follow_type_pack_id(*generic_pack);
            if !seen.insert(generic_pack) {
                return false;
            }

            if !counter.hit_limits {
                if let Some(state) = counter.generic_packs.get(&generic_pack) {
                    if state.count == 0 {
                        return false;
                    }
                }
            }

            !get_type_pack_id::<GenericTypePack>(generic_pack).is_null()
        });
    }
}

struct GenericCounterLocal {
    /// C++ `GenericTypeVisitor::seen` (VisitType.h:76) — the recursion-stack
    /// guard inherited by `GenericCounter : TypeVisitor`. Distinct from
    /// `seen_counts`: this set is cleared on the way back up (`unsee`) so the
    /// same type can be counted via multiple sibling paths, but a type that is
    /// currently *on the stack* (a cycle, e.g. `t1 = Instance & { IsA: (t1,
    /// ...) }`) is not re-entered — without it the recursive type is traversed
    /// repeatedly at flipped polarity, double-counting generics and forcing
    /// their polarity to `Mixed`.
    seen: HashSet<*const core::ffi::c_void>,
    seen_counts: HashMap<TypeId, usize>,
    seen_pack_counts: HashMap<TypePackId, usize>,
    generics: HashMap<TypeId, CounterState>,
    generic_packs: HashMap<TypePackId, CounterState>,
    polarity: Polarity,
    steps: i32,
    hit_limits: bool,
}

impl GenericCounterLocal {
    fn new(cached_types: *mut DenseHashSet<TypeId>) -> Self {
        let _ = cached_types;
        Self {
            seen: HashSet::new(),
            seen_counts: HashMap::new(),
            seen_pack_counts: HashMap::new(),
            generics: HashMap::new(),
            generic_packs: HashMap::new(),
            polarity: Polarity::Positive,
            steps: 0,
            hit_limits: false,
        }
    }

    fn check_limits(&mut self) -> bool {
        self.steps += 1;
        self.hit_limits |= self.steps > FInt::LuauGenericCounterMaxSteps.get();
        !self.hit_limits
    }

    fn traverse_type_id(&mut self, ty: TypeId) {
        if ty.is_null() || !self.check_limits() {
            return;
        }

        let ty = unsafe { follow_type_id(ty) };

        // C++ `GenericTypeVisitor::traverse` recursion-stack `seen` guard
        // (VisitType.h:235): skip a type already on the stack, then `unsee` it
        // afterwards so sibling paths still count it.
        let key = ty as *const core::ffi::c_void;
        if !self.seen.insert(key) {
            return;
        }
        self.dispatch_type_id(ty);
        self.seen.remove(&key);
    }

    fn dispatch_type_id(&mut self, ty: TypeId) {
        unsafe {
            if !get_type_id::<GenericType>(ty).is_null() {
                if let Some(state) = self.generics.get_mut(&ty) {
                    state.count += 1;
                    state.polarity = state.polarity | self.polarity;
                }
                return;
            }

            if !get_type_id::<ExternType>(ty).is_null() {
                return;
            }

            if let Some(ft) = get_type_id::<FunctionType>(ty).as_ref() {
                if (*ty).persistent {
                    return;
                }

                let seen_count = self.seen_counts.entry(ty).or_default();
                if *seen_count > 1 {
                    return;
                }
                *seen_count += 1;

                self.polarity = invert(self.polarity);
                self.traverse_type_pack_id(ft.arg_types);
                self.polarity = invert(self.polarity);
                self.traverse_type_pack_id(ft.ret_types);
                return;
            }

            if let Some(tt) = get_type_id::<TableType>(ty).as_ref() {
                if (*ty).persistent {
                    return;
                }

                let seen_count = self.seen_counts.entry(ty).or_default();
                if *seen_count > 1 {
                    return;
                }
                *seen_count += 1;

                let previous = self.polarity;
                for prop in tt.props.values() {
                    if prop.is_read_only() {
                        if let Some(read_ty) = prop.read_ty {
                            self.traverse_type_id(read_ty);
                        }
                    } else if prop.is_write_only() {
                        let p = self.polarity;
                        self.polarity = Polarity::Negative;
                        if let Some(write_ty) = prop.write_ty {
                            self.traverse_type_id(write_ty);
                        }
                        self.polarity = p;
                    } else if prop.is_shared() {
                        let p = self.polarity;
                        self.polarity = Polarity::Mixed;
                        if let Some(read_ty) = prop.read_ty {
                            self.traverse_type_id(read_ty);
                        }
                        self.polarity = p;
                    } else {
                        if let Some(read_ty) = prop.read_ty {
                            self.traverse_type_id(read_ty);
                        }

                        let p = self.polarity;
                        self.polarity = Polarity::Negative;
                        if let Some(write_ty) = prop.write_ty {
                            self.traverse_type_id(write_ty);
                        }
                        self.polarity = p;
                    }
                }

                if let Some(indexer) = &tt.indexer {
                    self.polarity = Polarity::Mixed;
                    self.traverse_type_id(indexer.index_type);
                    self.traverse_type_id(indexer.index_result_type);
                    self.polarity = previous;
                }
                return;
            }

            if let Some(ft) = get_type_id::<FreeType>(ty).as_ref() {
                self.traverse_type_id(ft.lower_bound);
                self.traverse_type_id(ft.upper_bound);
            } else if let Some(tfit) = get_type_id::<TypeFunctionInstanceType>(ty).as_ref() {
                let seen_count = self.seen_counts.entry(ty).or_default();
                if *seen_count > 1 {
                    return;
                }
                *seen_count += 1;

                for &arg in &tfit.type_arguments {
                    self.traverse_type_id(arg);
                }

                for &arg_pack in &tfit.pack_arguments {
                    self.traverse_type_pack_id(arg_pack);
                }
            } else if let Some(ut) = get_type_id::<UnionType>(ty).as_ref() {
                for &option in &ut.options {
                    self.traverse_type_id(option);
                }
            } else if let Some(it) = get_type_id::<IntersectionType>(ty).as_ref() {
                for &part in &it.parts {
                    self.traverse_type_id(part);
                }
            } else if let Some(mt) = get_type_id::<MetatableType>(ty).as_ref() {
                self.traverse_type_id(mt.table);
                self.traverse_type_id(mt.metatable);
            } else if let Some(nt) = get_type_id::<NegationType>(ty).as_ref() {
                self.traverse_type_id(nt.ty);
            }
        }
    }

    fn traverse_type_pack_id(&mut self, tp: TypePackId) {
        if tp.is_null() || !self.check_limits() {
            return;
        }

        let tp = unsafe { follow_type_pack_id(tp) };

        let key = tp as *const core::ffi::c_void;
        if !self.seen.insert(key) {
            return;
        }
        self.dispatch_type_pack_id(tp);
        self.seen.remove(&key);
    }

    fn dispatch_type_pack_id(&mut self, tp: TypePackId) {
        unsafe {
            if let Some(generic_pack) = get_type_pack_id::<GenericTypePack>(tp).as_ref() {
                let _ = generic_pack;
                if let Some(state) = self.generic_packs.get_mut(&tp) {
                    state.count += 1;
                    state.polarity = state.polarity | self.polarity;
                }
                return;
            }

            let seen_count = self.seen_pack_counts.entry(tp).or_default();
            if *seen_count > 1 {
                return;
            }
            *seen_count += 1;

            if let Some(pack) = get_type_pack_id::<TypePack>(tp).as_ref() {
                for &head in &pack.head {
                    self.traverse_type_id(head);
                }

                if let Some(tail) = pack.tail {
                    self.traverse_type_pack_id(tail);
                }
            } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp).as_ref() {
                self.traverse_type_id(vtp.ty);
            }
        }
    }
}
