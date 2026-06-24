use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::normalizer::Normalizer;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::records::type_pack_var::TypePackVar;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl Normalizer {
    pub fn intersection_of_type_packs_internal(
        &mut self,
        here: TypePackId,
        there: TypePackId,
    ) -> Option<TypePackId> {
        self.consume_fuel();

        if here == there {
            return Some(here);
        }

        let mut head: Vec<TypeId> = Vec::new();
        let mut tail: Option<TypePackId> = None;

        let mut here_sub_there = true;
        let mut there_sub_here = true;

        let mut ith = begin_type_pack_id(here);
        let mut itt = begin_type_pack_id(there);
        let end_ith = end_type_pack_id(here);
        let end_itt = end_type_pack_id(there);

        while ith.operator_ne(&end_ith) && itt.operator_ne(&end_itt) {
            let hty = *ith.operator_deref();
            let tty = *itt.operator_deref();
            let ty = self.intersection_type(hty, tty);
            if ty != hty {
                here_sub_there = false;
            }
            if ty != tty {
                there_sub_here = false;
            }
            head.push(ty);
            ith.operator_inc();
            itt.operator_inc();
        }

        let mut deal_with_different_arities = |ith: &mut TypePackIterator,
                                               itt: TypePackIterator,
                                               here: TypePackId,
                                               _there: TypePackId,
                                               here_sub_there: &mut bool,
                                               there_sub_here: &mut bool|
         -> bool {
            if ith.operator_ne(&end_type_pack_id(here)) {
                let mut tty = unsafe { (*self.builtin_types).nilType };
                if let Some(ttail) = itt.tail() {
                    let p = unsafe { get_type_pack_id::<VariadicTypePack>(ttail) };
                    if !p.is_null() {
                        tty = unsafe { &*p }.ty;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }

                while ith.operator_ne(&end_type_pack_id(here)) {
                    let hty = *ith.operator_deref();
                    let ty = self.intersection_type(hty, tty);
                    if ty != hty {
                        *here_sub_there = false;
                    }
                    if ty != tty {
                        *there_sub_here = false;
                    }
                    head.push(ty);
                    ith.operator_inc();
                }
            }
            true
        };

        if !deal_with_different_arities(
            &mut ith,
            itt.clone(),
            here,
            there,
            &mut here_sub_there,
            &mut there_sub_here,
        ) {
            return None;
        }

        if !deal_with_different_arities(
            &mut itt,
            ith.clone(),
            there,
            here,
            &mut there_sub_here,
            &mut here_sub_there,
        ) {
            return None;
        }

        let htail = ith.tail();
        let ttail = itt.tail();

        if let Some(htail_val) = htail {
            if let Some(ttail_val) = ttail {
                if htail_val == ttail_val {
                    tail = Some(htail_val);
                } else {
                    let hvtp = unsafe { get_type_pack_id::<VariadicTypePack>(htail_val) };
                    let tvtp = unsafe { get_type_pack_id::<VariadicTypePack>(ttail_val) };

                    if !hvtp.is_null() && !tvtp.is_null() {
                        let hvtp_ref = unsafe { &*hvtp };
                        let tvtp_ref = unsafe { &*tvtp };

                        let ty = self.intersection_type(hvtp_ref.ty, tvtp_ref.ty);
                        if ty != hvtp_ref.ty {
                            there_sub_here = false;
                        }
                        if ty != tvtp_ref.ty {
                            here_sub_there = false;
                        }
                        let hidden = hvtp_ref.hidden & tvtp_ref.hidden;
                        tail = Some(unsafe {
                            (*self.arena).add_type_pack_t(VariadicTypePack { ty, hidden })
                        });
                    } else {
                        return None;
                    }
                }
            } else {
                let hvtp = unsafe { get_type_pack_id::<VariadicTypePack>(htail_val) };
                if !hvtp.is_null() {
                    here_sub_there = false;
                } else {
                    return None;
                }
            }
        } else if let Some(ttail_val) = ttail {
            let tvtp = unsafe { get_type_pack_id::<VariadicTypePack>(ttail_val) };
            if !tvtp.is_null() {
                there_sub_here = false;
            } else {
                return None;
            }
        }

        if here_sub_there {
            return Some(here);
        } else if there_sub_here {
            return Some(there);
        }

        if !head.is_empty() {
            return Some(unsafe { (*self.arena).add_type_pack_t(TypePack { head, tail }) });
        } else if let Some(t) = tail {
            return Some(t);
        } else {
            return Some(unsafe {
                (*self.arena).add_type_pack_t(TypePack {
                    head: Vec::new(),
                    tail: None,
                })
            });
        }
    }
}
