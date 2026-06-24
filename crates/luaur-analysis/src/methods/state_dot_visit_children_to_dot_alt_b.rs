use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::error_type_pack::ErrorTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::state_dot::StateDot;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::functions::format_append::formatAppend;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl StateDot {
    pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, index: i32) {
        if self.seen_tp.contains(&tp) {
            return;
        }
        self.seen_tp.insert(tp);

        self.start_node(index);
        self.start_node_label();

        unsafe {
            let btp = get_type_pack_id::<BoundTypePack>(tp);

            if !btp.is_null() {
                let btp = &*btp;
                formatAppend(&mut self.result, format_args!("BoundTypePack {}", index));
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();

                self.visit_child_type_pack_id_i32_c_char(btp.boundTo, index, core::ptr::null());
            } else if !get_type_pack_id::<TypePack>(tp).is_null() {
                let tpp = &*get_type_pack_id::<TypePack>(tp);
                formatAppend(&mut self.result, format_args!("TypePack {}", index));
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();

                for tv in &tpp.head {
                    self.visit_child_type_id_i32_c_char(*tv, index, core::ptr::null());
                }
                if let Some(tail) = tpp.tail {
                    self.visit_child_type_pack_id_i32_c_char(tail, index, c"tail".as_ptr());
                }
            } else if !get_type_pack_id::<VariadicTypePack>(tp).is_null() {
                let vtp = &*get_type_pack_id::<VariadicTypePack>(tp);
                formatAppend(
                    &mut self.result,
                    format_args!(
                        "VariadicTypePack {}{}",
                        if vtp.hidden { "hidden " } else { "" },
                        index
                    ),
                );
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();

                self.visit_child_type_id_i32_c_char(vtp.ty, index, core::ptr::null());
            } else if !get_type_pack_id::<FreeTypePack>(tp).is_null() {
                formatAppend(&mut self.result, format_args!("FreeTypePack {}", index));
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();
            } else if !get_type_pack_id::<GenericTypePack>(tp).is_null() {
                let gtp = &*get_type_pack_id::<GenericTypePack>(tp);
                if gtp.explicitName {
                    formatAppend(
                        &mut self.result,
                        format_args!("GenericTypePack {}", gtp.name),
                    );
                } else {
                    formatAppend(&mut self.result, format_args!("GenericTypePack {}", index));
                }
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();
            } else if !get_type_pack_id::<ErrorTypePack>(tp).is_null() {
                formatAppend(&mut self.result, format_args!("ErrorTypePack {}", index));
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();
            } else {
                LUAU_ASSERT!(false);
                self.finish_node_label_type_pack_id(tp);
                self.finish_node();
            }
        }
    }
}
