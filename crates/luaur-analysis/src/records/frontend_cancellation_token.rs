#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct FrontendCancellationToken {
    pub cancelled: core::sync::atomic::AtomicBool,
}

// `cancel` / `requested` live in their own method node files
// (methods/frontend_cancellation_token_{cancel,requested}.rs).

impl Clone for FrontendCancellationToken {
    fn clone(&self) -> Self {
        Self {
            cancelled: core::sync::atomic::AtomicBool::new(
                self.cancelled.load(core::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

impl Default for FrontendCancellationToken {
    fn default() -> Self {
        Self {
            cancelled: core::sync::atomic::AtomicBool::new(false),
        }
    }
}
