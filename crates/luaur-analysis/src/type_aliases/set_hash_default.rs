#[allow(non_camel_case_types)]
pub type SetHashDefault<T> = core::marker::PhantomData<T>;

/// In Luau's C++ source, `SetHashDefault` is a template alias that selects `DenseHashPointer`
/// if `T` is a pointer, otherwise `std::hash<T>`.
///
/// In Rust, because we cannot easily perform compile-time type dispatch in a type alias
/// without unstable features (specialization), and because `Set` (the primary consumer)
/// is typically instantiated with concrete types, this alias is provided as a marker.
///
/// When translating a `Set<T, Hash>`, if `Hash` is `SetHashDefault`, the implementation
/// should use `luaur_common::records::dense_hash_pointer::DenseHashPointer` if `T` is a
/// pointer type, or the default hasher otherwise.
#[allow(dead_code)]
type _SetHashDefaultCheck<T> = SetHashDefault<T>;

// Note: The C++ implementation uses:
// using SetHashDefault = std::conditional_t<std::is_pointer_v<T>, DenseHashPointer, std::hash<T>>;
