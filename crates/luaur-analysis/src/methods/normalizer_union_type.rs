use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_type(&mut self, mut here: TypeId, mut there: TypeId) -> TypeId {
        self.consume_fuel();

        unsafe {
            here = follow_type_id(here);
            there = follow_type_id(there);
        }

        if here == there {
            return here;
        }

        let here_never = !unsafe { get_type_id::<NeverType>(here) }.is_null();
        let there_any = !unsafe { get_type_id::<AnyType>(there) }.is_null();
        if here_never || there_any {
            return there;
        }

        let there_never = !unsafe { get_type_id::<NeverType>(there) }.is_null();
        let here_any = !unsafe { get_type_id::<AnyType>(here) }.is_null();
        if there_never || here_any {
            return here;
        }

        let mut tmps = TypeIds::type_ids();

        unsafe {
            let utv = get_type_id::<UnionType>(here);
            if !utv.is_null() {
                let mut heres = TypeIds::type_ids();
                for &ty in &(*utv).options {
                    heres.insert_type_id(ty);
                    tmps.insert_type_id(ty);
                }
                let key = self.cache_type_ids(heres);
                self.cached_unions.insert(key, here);
            } else {
                tmps.insert_type_id(here);
            }

            let utv = get_type_id::<UnionType>(there);
            if !utv.is_null() {
                let mut theres = TypeIds::type_ids();
                for &ty in &(*utv).options {
                    theres.insert_type_id(ty);
                    tmps.insert_type_id(ty);
                }
                let key = self.cache_type_ids(theres);
                self.cached_unions.insert(key, there);
            } else {
                tmps.insert_type_id(there);
            }
        }

        let cache_key = self.cache_type_ids(tmps.clone());
        if let Some(&cached) = self.cached_unions.get(&cache_key) {
            return cached;
        }

        let parts: Vec<TypeId> = tmps.order.clone();

        let result = unsafe { (*self.arena).add_type(UnionType { options: parts }) };
        self.cached_unions.insert(cache_key, result);

        result
    }
}
