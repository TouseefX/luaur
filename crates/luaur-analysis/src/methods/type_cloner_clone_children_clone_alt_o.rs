use crate::records::extern_type::ExternType;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::nominal_relation::NominalRelation;
use luaur_common::FFlag;

impl TypeCloner {
    pub fn clone_children_extern_type(&mut self, t: *mut ExternType) {
        unsafe {
            // `for (auto& [_, p] : t->props) p = shallowClone(p);`
            let keys: alloc::vec::Vec<_> = (*t).props.keys().cloned().collect();
            for key in keys {
                let p = (*t).props.get(&key).unwrap().clone();
                let cloned = self.shallow_clone_property(&p);
                (*t).props.insert(key, cloned);
            }

            if let Some(parent) = (*t).parent {
                (*t).parent = Some(self.shallow_clone_type_id(parent));
            }

            if let Some(metatable) = (*t).metatable {
                (*t).metatable = Some(self.shallow_clone_type_id(metatable));
            }

            if let Some(indexer) = &mut (*t).indexer {
                indexer.index_type = self.shallow_clone_type_id(indexer.index_type);
                indexer.index_result_type = self.shallow_clone_type_id(indexer.index_result_type);
            }

            if FFlag::DebugLuauUserDefinedClasses.get() {
                if let Some(relation) = &mut (*t).relation {
                    match relation {
                        NominalRelation::V0(obj) => {
                            obj.ty = self.shallow_clone_type_id(obj.ty);
                        }
                        NominalRelation::V1(klass) => {
                            klass.ty = self.shallow_clone_type_id(klass.ty);
                        }
                    }
                }
            }
        }
    }
}
