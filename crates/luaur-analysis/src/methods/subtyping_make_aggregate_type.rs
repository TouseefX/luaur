//! Faithful port of `Subtyping::makeAggregateType<T, Container>`
//! (Analysis/src/Subtyping.cpp:2918-2927).
use crate::records::r#type::Type;
use crate::records::subtyping::Subtyping;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    /// C++:
    /// ```cpp
    /// template<typename T, typename Container>
    /// TypeId Subtyping::makeAggregateType(const Container& container, TypeId orElse)
    /// {
    ///     if (container.empty())
    ///         return orElse;
    ///     else if (container.size() == 1)
    ///         return *begin(container);
    ///     else
    ///         return arena->addType(T{std::vector<TypeId>(begin(container), end(container))});
    /// }
    /// ```
    ///
    /// `T` is the aggregate type variant (e.g. `UnionType` / `IntersectionType`)
    /// constructed from the gathered `Vec<TypeId>`; `Container` is any sequence
    /// of `TypeId` (mirrors the C++ `begin(container)`/`end(container)` iteration).
    pub fn make_aggregate_type<T, Container>(
        &mut self,
        container: &Container,
        or_else: TypeId,
    ) -> TypeId
    where
        Container: AsRef<[TypeId]>,
        T: From<alloc::vec::Vec<TypeId>> + Into<Type> + 'static,
    {
        let elements = container.as_ref();

        if elements.is_empty() {
            or_else
        } else if elements.len() == 1 {
            elements[0]
        } else {
            let value = T::from(elements.to_vec());
            unsafe { (*self.arena).add_type(value) }
        }
    }
}
