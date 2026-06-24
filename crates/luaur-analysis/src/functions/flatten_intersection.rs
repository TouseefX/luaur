//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:134:flatten_intersection`
//! Source: `Analysis/src/Type.cpp:134-164` (hand-ported)

use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

pub fn flatten_intersection(ty: TypeId) -> Vec<TypeId> {
    unsafe {
        if get::<IntersectionType>(follow(ty)).is_null() {
            return vec![ty];
        }

        let mut seen = std::collections::HashSet::<TypeId>::new();
        let mut queue: VecDeque<TypeId> = VecDeque::from(vec![ty]);

        let mut result = Vec::new();

        while let Some(front) = queue.pop_front() {
            let current = follow(front);

            if seen.contains(&current) {
                continue;
            }

            seen.insert(current);

            let itv = get::<IntersectionType>(current);
            if !itv.is_null() {
                for &t in (*itv).parts.iter() {
                    queue.push_back(t);
                }
            } else {
                result.push(current);
            }
        }

        result
    }
}
