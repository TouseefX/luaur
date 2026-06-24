use crate::records::frontend_cancellation_token::FrontendCancellationToken;
use alloc::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct TypeCheckLimits {
    pub(crate) finishTime: Option<f64>,
    pub(crate) instantiationChildLimit: Option<i32>,
    pub(crate) unifierIterationLimit: Option<i32>,
    pub(crate) cancellationToken: Option<Arc<FrontendCancellationToken>>,
}

#[allow(non_snake_case)]
impl TypeCheckLimits {
    pub fn finishTime(&self) -> Option<f64> {
        self.finishTime
    }

    pub fn instantiationChildLimit(&self) -> Option<i32> {
        self.instantiationChildLimit
    }

    pub fn unifierIterationLimit(&self) -> Option<i32> {
        self.unifierIterationLimit
    }

    pub fn cancellationToken(&self) -> Option<Arc<FrontendCancellationToken>> {
        self.cancellationToken.clone()
    }
}
