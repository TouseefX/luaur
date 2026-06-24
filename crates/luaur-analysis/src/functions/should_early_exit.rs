use crate::enums::normalization_result::NormalizationResult;

pub fn should_early_exit(res: NormalizationResult) -> bool {
    // if res is hit limits, return control flow
    if res == NormalizationResult::HitLimits || res == NormalizationResult::False {
        return true;
    }
    false
}
