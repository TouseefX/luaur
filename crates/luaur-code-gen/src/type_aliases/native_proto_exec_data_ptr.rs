#[allow(non_camel_case_types)]
pub type NativeProtoExecDataPtr = core::ptr::NonNull<u32>;

// Note: In the Luau C++ source, NativeProtoExecDataPtr is a std::unique_ptr with a custom deleter.
// Since this is a native-only type used for managing raw memory allocated for native execution data,
// and Rust's Box<[u32]> does not support custom deleters in the same way without a custom allocator,
// downstream code in this crate manages the lifecycle via the NativeProtoExecDataDeleter manually
// or via the SharedCodeAllocator. For the type alias itself, we represent the pointer.
