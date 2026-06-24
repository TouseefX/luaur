use crate::records::frontend_cancellation_token::FrontendCancellationToken;
use core::sync::atomic::Ordering;

impl FrontendCancellationToken {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}
