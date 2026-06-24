//! Faithful port of `Subtyping::isSubTailCovariantWith`
//! (Analysis/src/Subtyping.cpp:1180-1258).
use crate::enums::early_exit::EarlyExit;
use crate::enums::pack_field::PackField;
use crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy;
use crate::enums::variant::Variant;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::slice_type_pack::slice_type_pack;
use crate::records::error_type_pack::ErrorTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::index::Index;
use crate::records::pack_slice::PackSlice;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_error::TypeError;
use crate::records::type_pack::TypePack;
use crate::records::unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::component::Component;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::lookup_result::LookupResult;
use crate::type_aliases::path::Path;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::methods::path_builder_build::PathBuilderBuild;
use crate::methods::path_builder_tail::PathBuilderTail;
use crate::methods::path_builder_variadic::PathBuilderVariadic;
use crate::records::path_builder::PathBuilder;

impl Subtyping {
    pub fn is_sub_tail_covariant_with(
        &mut self,
        env: &mut SubtypingEnvironment,
        output_result: &mut SubtypingResult,
        sub_tp: TypePackId,
        sub_tail: TypePackId,
        super_tp: TypePackId,
        super_head_start_index: usize,
        super_head: &Vec<TypeId>,
        super_tail: Option<TypePackId>,
        scope: *mut Scope,
    ) -> EarlyExit {
        let _ = sub_tp;

        if let Some(vt) = unsafe { get_type_pack_id::<VariadicTypePack>(sub_tail).as_ref() } {
            for i in super_head_start_index..super_head.len() {
                let mut next = self
                    .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                        env,
                        vt.ty,
                        super_head[i],
                        scope,
                    );
                next.with_sub_path(
                    PathBuilder {
                        components: alloc::vec::Vec::new(),
                    }
                    .tail()
                    .variadic()
                    .build(),
                );
                next.with_super_component(Component::Index(Index {
                    index: i,
                    variant: Variant::Pack,
                }));
                output_result.and_also(next, SubtypingSuppressionPolicy::Any);
            }
            EarlyExit::No
        } else if !unsafe { get_type_pack_id::<GenericTypePack>(sub_tail).is_null() } {
            let lookup_result = env.lookup_generic_pack(sub_tail);
            let result: SubtypingResult;
            if let LookupResult::V2(_) = lookup_result {
                // get_if<MappedGenericEnvironment::NotBindable>
                let mut r = SubtypingResult {
                    is_subtype: false,
                    normalization_too_complex: false,
                    is_cacheable: false,
                    ..Default::default()
                };
                r.with_sub_component(Component::PackField(PackField::Tail));
                r.with_super_component(Component::PackSlice(PackSlice {
                    start_index: super_head_start_index,
                }));
                result = r;
            } else {
                let super_tail_pack = slice_type_pack(
                    super_head_start_index,
                    super_tp,
                    super_head,
                    super_tail,
                    unsafe { &*self.builtin_types },
                    unsafe { &mut *self.arena },
                );

                if let LookupResult::V0(mapped_gen) = lookup_result {
                    // get_if<TypePackId> — subtype against the mapped generic pack.
                    let mut sub_tp_to_compare = mapped_gen;

                    // If mappedGen has a hidden variadic tail, we clip it for better
                    // arity mismatch reporting.
                    let tp = unsafe { get_type_pack_id::<TypePack>(mapped_gen).as_ref() };
                    if let Some(tp) = tp {
                        if let Some(tail) = tp.tail {
                            let vtp = unsafe {
                                get_type_pack_id::<VariadicTypePack>(follow_type_pack_id(tail))
                                    .as_ref()
                            };
                            if let Some(vtp) = vtp {
                                if vtp.hidden {
                                    sub_tp_to_compare = unsafe {
                                        (*self.arena)
                                            .add_type_pack_initializer_list_type_id(&tp.head)
                                    };
                                }
                            }
                        }
                    }

                    let mut r = self
                        .is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
                            env,
                            sub_tp_to_compare,
                            super_tail_pack,
                            scope,
                        );
                    r.with_sub_path(Path::from_components(alloc::vec![
                        Component::PackField(PackField::Tail),
                        Component::GenericPackMapping(GenericPackMapping {
                            mappedType: mapped_gen
                        }),
                    ]));
                    r.with_super_component(Component::PackSlice(PackSlice {
                        start_index: super_head_start_index,
                    }));
                    result = r;
                } else {
                    // get_if<MappedGenericEnvironment::Unmapped>
                    let ok = env
                        .mapped_generic_packs
                        .bind_generic(sub_tail, super_tail_pack);
                    let mut r = SubtypingResult {
                        is_subtype: ok,
                        normalization_too_complex: false,
                        is_cacheable: false,
                        ..Default::default()
                    };
                    r.with_sub_component(Component::PackField(PackField::Tail));
                    r.with_super_component(Component::PackSlice(PackSlice {
                        start_index: super_head_start_index,
                    }));
                    result = r;
                }
            }

            output_result.and_also(result, SubtypingSuppressionPolicy::Any);
            EarlyExit::Yes
        } else if !unsafe { get_type_pack_id::<ErrorTypePack>(sub_tail).is_null() } {
            let mut r = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
            r.with_sub_component(Component::PackField(PackField::Tail));
            *output_result = r;
            EarlyExit::Yes
        } else if !unsafe { get_type_pack_id::<FreeTypePack>(sub_tail).is_null() } {
            let super_tail_pack = slice_type_pack(
                super_head_start_index,
                super_tp,
                super_head,
                super_tail,
                unsafe { &*self.builtin_types },
                unsafe { &mut *self.arena },
            );
            let mut r = SubtypingResult {
                is_subtype: true,
                ..Default::default()
            };
            r.with_sub_component(Component::PackField(PackField::Tail));
            r.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
                sub_pack: sub_tail,
                super_pack: super_tail_pack,
                returns: false,
            }));
            output_result.and_also(r, SubtypingSuppressionPolicy::Any);
            EarlyExit::Yes
        } else {
            let mut r = SubtypingResult {
                is_subtype: false,
                ..Default::default()
            };
            r.with_sub_component(Component::PackField(PackField::Tail));
            r.with_error(TypeError::type_error_location_type_error_data(
                unsafe { (*scope).location.clone() },
                UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
            ));
            *output_result = r;
            EarlyExit::Yes
        }
    }
}
