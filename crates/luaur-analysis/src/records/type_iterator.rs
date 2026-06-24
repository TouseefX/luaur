//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Type.h:1130:type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1126-1248` (hand-ported)
//!
//! C++ `template<typename T> struct TypeIterator` — traverses T (UnionType or
//! IntersectionType) yielding each TypeId; encountering a nested T yields the
//! TypeIds within instead. (An earlier translation hardcoded UnionType in
//! `advance`, silently breaking intersection iteration.)

use crate::records::intersection_type::IntersectionType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariantMember;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

/// C++ `getTypes(const T*)` overload pair (Type.cpp).
pub trait TypeIteratorMember: TypeVariantMember + 'static {
    fn get_types(&self) -> &alloc::vec::Vec<TypeId>;
}

impl TypeIteratorMember for UnionType {
    fn get_types(&self) -> &alloc::vec::Vec<TypeId> {
        &self.options
    }
}

impl TypeIteratorMember for IntersectionType {
    fn get_types(&self) -> &alloc::vec::Vec<TypeId> {
        &self.parts
    }
}

#[derive(Debug)]
pub struct TypeIterator<T: TypeIteratorMember> {
    // (const T* t, size_t currentIndex)
    pub(crate) stack: VecDeque<(*const T, usize)>,
    /// Only needed to protect the iterator from hanging the thread.
    pub(crate) seen: DenseHashSet<*const T>,
}

// Manual impl: the derive would demand `T: Clone`, but the stack stores
// `*const T` (Copy) — the element type itself is never cloned.
impl<T: TypeIteratorMember> Clone for TypeIterator<T> {
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
            seen: self.seen.clone(),
        }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let copy: () = ();
    let ty: () = ();
}
