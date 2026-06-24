use crate::records::to_dot_options::ToDotOptions;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct StateDot {
    pub(crate) opts: ToDotOptions,
    pub(crate) seen_ty: DenseHashSet<TypeId>,
    pub(crate) seen_tp: DenseHashSet<TypePackId>,
    pub(crate) ty_to_index: DenseHashMap<TypeId, i32>,
    pub(crate) tp_to_index: DenseHashMap<TypePackId, i32>,
    pub(crate) next_index: i32,
    pub(crate) result: String,
}

impl StateDot {
    pub fn new(opts: ToDotOptions) -> Self {
        Self {
            opts,
            seen_ty: DenseHashSet::new(core::ptr::null_mut()),
            seen_tp: DenseHashSet::new(core::ptr::null_mut()),
            ty_to_index: DenseHashMap::new(core::ptr::null_mut()),
            tp_to_index: DenseHashMap::new(core::ptr::null_mut()),
            next_index: 1,
            result: String::new(),
        }
    }
}
