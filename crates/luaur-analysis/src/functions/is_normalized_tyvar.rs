use crate::functions::is_plain_tyvar::is_plain_tyvar;
use crate::functions::is_shallow_inhabited::is_shallow_inhabited;
use crate::functions::tyvar_index::tyvar_index;
use crate::type_aliases::normalized_tyvars::NormalizedTyvars;

pub fn is_normalized_tyvar(tyvars: &NormalizedTyvars) -> bool {
    for (tyvar, intersect) in tyvars {
        if !is_plain_tyvar(*tyvar) {
            return false;
        }
        if !is_shallow_inhabited(intersect) {
            return false;
        }
        for (other, _) in &intersect.tyvars {
            if tyvar_index(*other) <= tyvar_index(*tyvar) {
                return false;
            }
        }
    }
    true
}
