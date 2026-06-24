use crate::records::frontend_options::FrontendOptions;
use crate::records::type_check_limits::TypeCheckLimits;
use luaur_common::functions::get_clock::get_clock;

pub fn make_type_check_limits(options: &FrontendOptions) -> TypeCheckLimits {
    let mut limits = TypeCheckLimits::default();

    if let Some(time_limit) = options.module_time_limit_sec {
        limits.finishTime = Some(get_clock() + time_limit);
    } else {
        limits.finishTime = None;
    }

    limits.cancellationToken = options.cancellation_token.clone();

    limits
}
