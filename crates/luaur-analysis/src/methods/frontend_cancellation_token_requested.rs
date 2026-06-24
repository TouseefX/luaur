use crate::records::frontend_cancellation_token::FrontendCancellationToken;

impl FrontendCancellationToken {
    pub fn requested(&self) -> bool {
        self.cancelled.load(core::sync::atomic::Ordering::Relaxed)
    }
}
