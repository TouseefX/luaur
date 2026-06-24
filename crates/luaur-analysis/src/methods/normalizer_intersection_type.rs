use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn intersection_type(&mut self, here: TypeId, there: TypeId) -> TypeId {
        self.consume_fuel();

        let here = unsafe { follow_type_id(here) };
        let there = unsafe { follow_type_id(there) };

        if here == there {
            return here;
        }

        if !unsafe { get_type_id::<NeverType>(here).is_null() }
            || !unsafe { get_type_id::<AnyType>(there).is_null() }
        {
            return here;
        }

        if !unsafe { get_type_id::<NeverType>(there).is_null() }
            || !unsafe { get_type_id::<AnyType>(here).is_null() }
        {
            return there;
        }

        let mut tmps = TypeIds::type_ids();

        if !unsafe { get_type_id::<IntersectionType>(here).is_null() } {
            let utv = unsafe { &*get_type_id::<IntersectionType>(here) };
            let mut heres = TypeIds::type_ids();
            for &ty in &utv.parts {
                heres.insert_type_id(ty);
                tmps.insert_type_id(ty);
            }
            let key = self.cache_type_ids(heres);
            self.cached_intersections.insert(key, here);
        } else {
            tmps.insert_type_id(here);
        }

        if !unsafe { get_type_id::<IntersectionType>(there).is_null() } {
            let utv = unsafe { &*get_type_id::<IntersectionType>(there) };
            let mut theres = TypeIds::type_ids();
            for &ty in &utv.parts {
                theres.insert_type_id(ty);
                tmps.insert_type_id(ty);
            }
            let key = self.cache_type_ids(theres);
            self.cached_intersections.insert(key, there);
        } else {
            tmps.insert_type_id(there);
        }

        if tmps.size() == 1 {
            return tmps.front();
        }

        let cache_key = self.cache_type_ids(tmps.clone());
        if let Some(&cached) = self.cached_intersections.get(&cache_key) {
            return cached;
        }

        let parts = tmps.order.clone();
        let result = unsafe { (*self.arena).add_type(IntersectionType { parts }) };
        self.cached_intersections.insert(cache_key, result);

        result
    }
}
