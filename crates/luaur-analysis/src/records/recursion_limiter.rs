#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct RecursionLimiter {
    pub(crate) base: crate::records::recursion_counter::RecursionCounter,
    pub(crate) native_stack_guard: crate::records::native_stack_guard::NativeStackGuard,
}
