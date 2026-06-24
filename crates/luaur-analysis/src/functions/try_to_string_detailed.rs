extern crate alloc;

use alloc::string::String;
use alloc::sync::Arc;

use crate::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
use crate::functions::to_string_detailed_to_string_alt_b::to_string_detailed_type_pack_id_to_string_options;
use crate::records::to_string_options::ToStringOptions;
use crate::records::to_string_result::ToStringResult;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[allow(non_snake_case)]
pub fn try_to_string_detailed<T>(
    scope: ScopePtr,
    ty: T,
    functionTypeArguments: bool,
) -> Option<String>
where
    T: TryToStringDetailed,
{
    let mut opts = ToStringOptions::to_string_options(false);
    opts.use_line_breaks = false;
    opts.hide_table_kind = true;
    opts.function_type_arguments = functionTypeArguments;
    opts.scope = Some(scope);

    let name = ty.to_string_detailed(&mut opts);

    if name.error() || name.invalid() || name.cycle() || name.truncated() {
        None
    } else {
        Some(name.name().to_string())
    }
}

trait TryToStringDetailed {
    fn to_string_detailed(self, opts: &mut ToStringOptions) -> ToStringResult;
}

impl TryToStringDetailed for TypeId {
    fn to_string_detailed(self, opts: &mut ToStringOptions) -> ToStringResult {
        to_string_detailed_type_id_to_string_options(self, opts)
    }
}

impl TryToStringDetailed for TypePackId {
    fn to_string_detailed(self, opts: &mut ToStringOptions) -> ToStringResult {
        to_string_detailed_type_pack_id_to_string_options(self, opts)
    }
}
