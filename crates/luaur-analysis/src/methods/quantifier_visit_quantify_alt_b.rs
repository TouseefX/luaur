use crate::enums::table_state::TableState;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::quantifier::Quantifier;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Quantifier {
    pub fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
        unsafe {
            LUAU_ASSERT!(!get_mutable_type_id::<TableType>(ty).is_null());
            let ttv_mut = &mut *get_mutable_type_id::<TableType>(ty);

            if ttv_mut.state == TableState::Generic {
                self.seen_generic_type = true;
            }

            if ttv_mut.state == TableState::Free {
                self.seen_mutable_type = true;
            }

            // C++ `Quantifier::visit(TypeId, const TableType&)` (Quantify.cpp:68)
            // gates on `level.subsumes(ttv.level)` — TypeLevel subsumption, the
            // same gate the free-type/free-pack overloads use. The
            // `subsumes(Scope*, Scope*)` member exists on the C++ struct but is
            // vestigial; porting the gate to it left `scope` null, so the gate
            // was always false and free tables were never quantified.
            if !self.level.subsumes(&ttv_mut.level) {
                if ttv_mut.state == TableState::Unsealed {
                    self.seen_mutable_type = true;
                }
                return false;
            }

            if ttv_mut.state == TableState::Free {
                ttv_mut.state = TableState::Generic;
                self.seen_generic_type = true;
            } else if ttv_mut.state == TableState::Unsealed {
                ttv_mut.state = TableState::Sealed;
            }

            ttv_mut.level = self.level;

            true
        }
    }
}
