//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/LValue.h:19:field`
//! Source: `Analysis/include/Luau/LValue.h` (LValue.h:19-27, hand-ported)

use crate::type_aliases::l_value::LValue;
use alloc::string::String;
use alloc::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub parent: Option<Arc<LValue>>, // shared_ptr<LValue>
    pub key: String,
}
