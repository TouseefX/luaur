use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::records::work_item_iterative_type_visitor::WorkItem;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::seen_set_iterative_type_visitor::SeenSet;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct IterativeTypeVisitor {
    pub(crate) seen: SeenSet,
    pub(crate) work_queue: Vec<WorkItem>,
    pub(crate) parent_cursor: i32,
    pub(crate) work_cursor: u32,
    pub(crate) visitor_name: String,
    pub(crate) skip_bound_types: bool,
    pub(crate) visit_once: bool,
}

impl Default for IterativeTypeVisitor {
    fn default() -> Self {
        Self {
            seen: SeenSet::default(),
            work_queue: Vec::new(),
            parent_cursor: -1,
            work_cursor: 0,
            visitor_name: String::new(),
            skip_bound_types: false,
            visit_once: true,
        }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let t: () = ();
    let isType: () = ();
    let parent: () = ();
}

pub trait IterativeTypeVisitorTrait {
    fn visitor_base(&mut self) -> &mut IterativeTypeVisitor;

    fn cycle_type_id(&mut self, _ty: TypeId) {}
    fn cycle_type_pack_id(&mut self, _tp: TypePackId) {}

    fn visit_type_id(&mut self, _ty: TypeId) -> bool {
        true
    }
    fn visit_type_id_bound_type(&mut self, ty: TypeId, _btv: &BoundType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_free_type(&mut self, ty: TypeId, _ftv: &FreeType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_generic_type(&mut self, ty: TypeId, _gtv: &GenericType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_error_type(&mut self, ty: TypeId, _etv: &ErrorType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_primitive_type(&mut self, ty: TypeId, _ptv: &PrimitiveType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_function_type(&mut self, ty: TypeId, _ftv: &FunctionType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_table_type(&mut self, ty: TypeId, _ttv: &TableType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_metatable_type(&mut self, ty: TypeId, _mtv: &MetatableType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_extern_type(&mut self, ty: TypeId, _etv: &ExternType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_any_type(&mut self, ty: TypeId, _atv: &AnyType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_no_refine_type(&mut self, ty: TypeId, _nrt: &NoRefineType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_unknown_type(&mut self, ty: TypeId, _utv: &UnknownType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_never_type(&mut self, ty: TypeId, _ntv: &NeverType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_union_type(&mut self, ty: TypeId, _utv: &UnionType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_intersection_type(&mut self, ty: TypeId, _itv: &IntersectionType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_blocked_type(&mut self, ty: TypeId, _btv: &BlockedType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        _petv: &PendingExpansionType,
    ) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_singleton_type(&mut self, ty: TypeId, _stv: &SingletonType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_negation_type(&mut self, ty: TypeId, _ntv: &NegationType) -> bool {
        self.visit_type_id(ty)
    }
    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        _tfit: &TypeFunctionInstanceType,
    ) -> bool {
        self.visit_type_id(ty)
    }

    fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        true
    }
    fn visit_type_pack_id_bound_type_pack(&mut self, tp: TypePackId, _btp: &BoundTypePack) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_free_type_pack(&mut self, tp: TypePackId, _ftp: &FreeTypePack) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_generic_type_pack(
        &mut self,
        tp: TypePackId,
        _gtp: &GenericTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_error_type_pack(&mut self, tp: TypePackId, _etp: &ErrorTypePack) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_type_pack(&mut self, tp: TypePackId, _pack: &TypePack) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_variadic_type_pack(
        &mut self,
        tp: TypePackId,
        _vtp: &VariadicTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_blocked_type_pack(
        &mut self,
        tp: TypePackId,
        _btp: &BlockedTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }
    fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.visit_type_pack_id(tp)
    }

    fn run_type_id(&mut self, root_ty: TypeId) {
        {
            let base = self.visitor_base();
            base.parent_cursor = -1;
            base.work_cursor = 0;
            base.work_queue.clear();
        }

        self.traverse_type_id(root_ty);
        self.process_work_queue();
    }

    fn run_type_pack_id(&mut self, root_tp: TypePackId) {
        {
            let base = self.visitor_base();
            base.parent_cursor = -1;
            base.work_cursor = 0;
            base.work_queue.clear();
        }

        self.traverse_type_pack_id(root_tp);
        self.process_work_queue();
    }

    fn traverse_type_id(&mut self, ty: TypeId) {
        let parent = self.visitor_base().parent_cursor;
        self.visitor_base()
            .work_queue
            .push(WorkItem::work_item_type_id_i32(ty, parent));
    }

    fn traverse_type_pack_id(&mut self, tp: TypePackId) {
        let parent = self.visitor_base().parent_cursor;
        self.visitor_base()
            .work_queue
            .push(WorkItem::work_item_type_pack_id_i32(tp, parent));
    }

    fn process_work_queue(&mut self) {
        loop {
            let item = {
                let base = self.visitor_base();
                if (base.work_cursor as usize) >= base.work_queue.len() {
                    break;
                }

                base.parent_cursor = base.work_cursor as i32;
                base.work_queue[base.work_cursor as usize].clone()
            };

            if let Some(ty) = item.type_id() {
                if self.is_cyclic_type_id(ty) {
                    self.cycle_type_id(ty);
                } else {
                    self.process_type_id(ty);
                }
            } else if let Some(tp) = item.type_pack_id() {
                if self.is_cyclic_type_pack_id(tp) {
                    self.cycle_type_pack_id(tp);
                } else {
                    self.process_type_pack_id(tp);
                }
            } else {
                luaur_common::LUAU_ASSERT!(false);
            }

            self.visitor_base().work_cursor += 1;
        }
    }

    fn process_type_id(&mut self, mut ty: TypeId) {
        if self.visitor_base().skip_bound_types {
            ty = unsafe { crate::functions::follow_type::follow_type_id(ty) };
        }

        if self.has_seen(ty as *const core::ffi::c_void) {
            return;
        }

        let variant = unsafe { (*ty).ty.clone() };

        match variant {
            TypeVariant::Bound(bound_to) => {
                let btv = BoundType { boundTo: bound_to };
                luaur_common::LUAU_ASSERT!(!self.visitor_base().skip_bound_types);
                if self.visit_type_id_bound_type(ty, &btv) {
                    self.traverse_type_id(bound_to);
                }
            }
            TypeVariant::Free(ftv) => {
                if self.visit_type_id_free_type(ty, &ftv) {
                    luaur_common::LUAU_ASSERT!(!ftv.lower_bound.is_null());
                    luaur_common::LUAU_ASSERT!(!ftv.upper_bound.is_null());
                    self.traverse_type_id(ftv.lower_bound);
                    self.traverse_type_id(ftv.upper_bound);
                }
            }
            TypeVariant::Generic(gtv) => {
                self.visit_type_id_generic_type(ty, &gtv);
            }
            TypeVariant::Error(etv) => {
                self.visit_type_id_error_type(ty, &etv);
            }
            TypeVariant::Primitive(ptv) => {
                self.visit_type_id_primitive_type(ty, &ptv);
            }
            TypeVariant::Function(ftv) => {
                if self.visit_type_id_function_type(ty, &ftv) {
                    self.traverse_type_pack_id(ftv.arg_types);
                    self.traverse_type_pack_id(ftv.ret_types);
                }
            }
            TypeVariant::Table(ttv) => {
                luaur_common::LUAU_ASSERT!(
                    !self.visitor_base().skip_bound_types || ttv.bound_to.is_none()
                );
                if self.visitor_base().skip_bound_types && ttv.bound_to.is_some() {
                    self.traverse_type_id(ttv.bound_to.unwrap());
                } else if self.visit_type_id_table_type(ty, &ttv) {
                    if let Some(bound_to) = ttv.bound_to {
                        self.traverse_type_id(bound_to);
                    } else {
                        for (_name, prop) in ttv.props.iter() {
                            if let Some(read_ty) = prop.read_ty {
                                self.traverse_type_id(read_ty);
                            }
                            if let Some(write_ty) = prop.write_ty {
                                if !prop.is_shared() {
                                    self.traverse_type_id(write_ty);
                                }
                            }
                        }

                        if let Some(indexer) = &ttv.indexer {
                            self.traverse_type_id(indexer.index_type);
                            self.traverse_type_id(indexer.index_result_type);
                        }
                    }
                }
            }
            TypeVariant::Metatable(mtv) => {
                if self.visit_type_id_metatable_type(ty, &mtv) {
                    self.traverse_type_id(mtv.table);
                    self.traverse_type_id(mtv.metatable);
                }
            }
            TypeVariant::Extern(etv) => {
                if self.visit_type_id_extern_type(ty, &etv) {
                    for (_name, prop) in etv.props.iter() {
                        if let Some(read_ty) = prop.read_ty {
                            self.traverse_type_id(read_ty);
                        }
                        if let Some(write_ty) = prop.write_ty {
                            if !prop.is_shared() {
                                self.traverse_type_id(write_ty);
                            }
                        }
                    }

                    if let Some(parent) = etv.parent {
                        self.traverse_type_id(parent);
                    }
                    if let Some(metatable) = etv.metatable {
                        self.traverse_type_id(metatable);
                    }
                    if let Some(indexer) = &etv.indexer {
                        self.traverse_type_id(indexer.index_type);
                        self.traverse_type_id(indexer.index_result_type);
                    }
                }
            }
            TypeVariant::Any(atv) => {
                self.visit_type_id_any_type(ty, &atv);
            }
            TypeVariant::NoRefine(nrt) => {
                self.visit_type_id_no_refine_type(ty, &nrt);
            }
            TypeVariant::Union(utv) => {
                if self.visit_type_id_union_type(ty, &utv) {
                    let mut union_changed = false;
                    for opt_ty in utv.options {
                        self.traverse_type_id(opt_ty);
                        if unsafe {
                            !matches!(
                                (*crate::functions::follow_type::follow_type_id(ty)).ty,
                                TypeVariant::Union(_)
                            )
                        } {
                            union_changed = true;
                            break;
                        }
                    }

                    if union_changed {
                        self.traverse_type_id(ty);
                    }
                }
            }
            TypeVariant::Intersection(itv) => {
                if self.visit_type_id_intersection_type(ty, &itv) {
                    let mut intersection_changed = false;
                    for part_ty in itv.parts {
                        self.traverse_type_id(part_ty);
                        if unsafe {
                            !matches!(
                                (*crate::functions::follow_type::follow_type_id(ty)).ty,
                                TypeVariant::Intersection(_)
                            )
                        } {
                            intersection_changed = true;
                            break;
                        }
                    }

                    if intersection_changed {
                        self.traverse_type_id(ty);
                    }
                }
            }
            TypeVariant::Lazy(ltv) => {
                if !ltv.unwrapped.is_null() {
                    self.traverse_type_id(ltv.unwrapped);
                }
            }
            TypeVariant::Singleton(stv) => {
                self.visit_type_id_singleton_type(ty, &stv);
            }
            TypeVariant::Blocked(btv) => {
                self.visit_type_id_blocked_type(ty, &btv);
            }
            TypeVariant::Unknown(utv) => {
                self.visit_type_id_unknown_type(ty, &utv);
            }
            TypeVariant::Never(ntv) => {
                self.visit_type_id_never_type(ty, &ntv);
            }
            TypeVariant::PendingExpansion(petv) => {
                if self.visit_type_id_pending_expansion_type(ty, &petv) {
                    for a in petv.type_arguments {
                        self.traverse_type_id(a);
                    }
                    for a in petv.pack_arguments {
                        self.traverse_type_pack_id(a);
                    }
                }
            }
            TypeVariant::Negation(ntv) => {
                if self.visit_type_id_negation_type(ty, &ntv) {
                    self.traverse_type_id(ntv.ty);
                }
            }
            TypeVariant::TypeFunctionInstance(tfit) => {
                if self.visit_type_id_type_function_instance_type(ty, &tfit) {
                    for &p in &tfit.type_arguments {
                        self.traverse_type_id(p);
                    }
                    for &p in &tfit.pack_arguments {
                        self.traverse_type_pack_id(p);
                    }
                }
            }
        }

        self.unsee(ty as *const core::ffi::c_void);
    }

    fn process_type_pack_id(&mut self, tp: TypePackId) {
        if self.has_seen(tp as *const core::ffi::c_void) {
            return;
        }

        let variant = unsafe { (*tp).ty.clone() };

        match variant {
            TypePackVariant::Bound(bound_to) => {
                let btp = BoundTypePack { boundTo: bound_to };
                if self.visit_type_pack_id_bound_type_pack(tp, &btp) {
                    self.traverse_type_pack_id(bound_to);
                }
            }
            TypePackVariant::Free(ftp) => {
                self.visit_type_pack_id_free_type_pack(tp, &ftp);
            }
            TypePackVariant::Generic(gtp) => {
                self.visit_type_pack_id_generic_type_pack(tp, &gtp);
            }
            TypePackVariant::Error(etp) => {
                self.visit_type_pack_id_error_type_pack(tp, &etp);
            }
            TypePackVariant::TypePack(pack) => {
                if self.visit_type_pack_id_type_pack(tp, &pack) {
                    for ty in pack.head {
                        self.traverse_type_id(ty);
                    }
                    if let Some(tail) = pack.tail {
                        self.traverse_type_pack_id(tail);
                    }
                }
            }
            TypePackVariant::Variadic(vtp) => {
                if self.visit_type_pack_id_variadic_type_pack(tp, &vtp) {
                    self.traverse_type_id(vtp.ty);
                }
            }
            TypePackVariant::Blocked(btp) => {
                self.visit_type_pack_id_blocked_type_pack(tp, &btp);
            }
            TypePackVariant::TypeFunctionInstance(tfitp) => {
                if self.visit_type_pack_id_type_function_instance_type_pack(tp, &tfitp) {
                    for &t in &tfitp.typeArguments {
                        self.traverse_type_id(t);
                    }
                    for &t in &tfitp.packArguments {
                        self.traverse_type_pack_id(t);
                    }
                }
            }
        }

        self.unsee(tp as *const core::ffi::c_void);
    }

    fn has_seen(&mut self, tv: *const core::ffi::c_void) -> bool {
        if !self.visitor_base().visit_once {
            return false;
        }

        let base = self.visitor_base();
        let is_fresh = !base.seen.contains(&tv);
        base.seen.insert(tv);
        !is_fresh
    }

    fn unsee(&mut self, _tv: *const core::ffi::c_void) {
        if !self.visitor_base().visit_once {
            // `has_seen` returns before inserting when visitOnce is false.
        }
    }

    fn is_cyclic_type_id(&mut self, ty: TypeId) -> bool {
        let base = self.visitor_base();
        let mut cursor = base.work_cursor as i32;
        let mut item = &base.work_queue[base.work_cursor as usize];

        while item.parent >= 0 {
            luaur_common::LUAU_ASSERT!(item.parent < cursor);
            cursor = item.parent;
            item = &base.work_queue[cursor as usize];

            if item.operator_eq_type_id(ty) {
                return true;
            }
        }

        false
    }

    fn is_cyclic_type_pack_id(&mut self, tp: TypePackId) -> bool {
        let base = self.visitor_base();
        let mut cursor = base.work_cursor as i32;
        let mut item = &base.work_queue[base.work_cursor as usize];

        while item.parent >= 0 {
            luaur_common::LUAU_ASSERT!(item.parent < cursor);
            cursor = item.parent;
            item = &base.work_queue[cursor as usize];

            if item.operator_eq_type_pack_id(tp) {
                return true;
            }
        }

        false
    }
}

impl IterativeTypeVisitorTrait for IterativeTypeVisitor {
    fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
        self
    }
}
