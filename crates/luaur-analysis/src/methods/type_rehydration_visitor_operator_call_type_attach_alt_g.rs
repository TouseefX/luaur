use crate::functions::allocate_string_type_attach::allocate_string_luau_allocator_string_view;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::table_type::TableType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::enums::ast_table_access::AstTableAccess;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_table_indexer::AstTableIndexer;
use luaur_ast::records::ast_table_prop::AstTableProp;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_17(&mut self, ttv: &TableType) -> *mut AstType {
        let _counter = RecursionCounter::recursion_counter_i32(
            &mut self.count as *mut i32 as *mut core::ffi::c_int,
        );

        if let Some(ref name) = ttv.name {
            if !self.options.banned_names.contains(name) {
                let params_size =
                    ttv.instantiated_type_params.len() + ttv.instantiated_type_pack_params.len();
                let params_data = unsafe {
                    (*self.allocator).allocate(core::mem::size_of::<AstTypeOrPack>() * params_size)
                        as *mut AstTypeOrPack
                };

                let mut idx = 0;
                for &ty_param in &ttv.instantiated_type_params {
                    let rehydrated = self.visit_type(ty_param);
                    unsafe {
                        core::ptr::write(
                            params_data.add(idx),
                            AstTypeOrPack {
                                r#type: rehydrated,
                                type_pack: core::ptr::null_mut(),
                            },
                        );
                    }
                    idx += 1;
                }

                for &tp_param in &ttv.instantiated_type_pack_params {
                    let rehydrated = self.rehydrate(tp_param);
                    unsafe {
                        core::ptr::write(
                            params_data.add(idx),
                            AstTypeOrPack {
                                r#type: core::ptr::null_mut(),
                                type_pack: rehydrated,
                            },
                        );
                    }
                    idx += 1;
                }

                let allocator = unsafe { &mut *self.allocator };
                let name_cstr = allocate_string_luau_allocator_string_view(allocator, name);
                let name_ast = AstName::ast_name_c_char(name_cstr);

                let parameters = AstArray {
                    data: params_data,
                    size: params_size,
                };

                let ref_node = AstTypeReference::new(
                    Location::default(),
                    None,
                    name_ast,
                    None,
                    Location::default(),
                    params_size != 0,
                    parameters,
                );

                return allocator.alloc(ref_node) as *mut AstType;
            }
        }

        if self.has_seen(ttv as *const TableType as *const core::ffi::c_void) {
            let allocator = unsafe { &mut *self.allocator };
            let name_cstr = if let Some(ref name) = ttv.name {
                allocate_string_luau_allocator_string_view(allocator, name)
            } else {
                allocate_string_luau_allocator_string_view(allocator, "<Cycle>")
            };
            let name_ast = AstName::ast_name_c_char(name_cstr);

            let ref_node = AstTypeReference::new(
                Location::default(),
                None,
                name_ast,
                None,
                Location::default(),
                false,
                AstArray {
                    data: core::ptr::null_mut(),
                    size: 0,
                },
            );

            return allocator.alloc(ref_node) as *mut AstType;
        }

        let props_size = ttv.props.len();
        let props_data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<AstTableProp>() * props_size)
                as *mut AstTableProp
        };

        let mut idx = 0;
        for (prop_name, prop) in &ttv.props {
            let _counter_inner = RecursionCounter::recursion_counter_i32(
                &mut self.count as *mut i32 as *mut core::ffi::c_int,
            );

            let name_cstr = {
                let allocator = unsafe { &mut *self.allocator };
                allocate_string_luau_allocator_string_view(allocator, prop_name)
            };
            let name_ast = AstName::ast_name_c_char(name_cstr);

            if prop.is_shared() {
                let read_ty_rehydrated = self.visit_type(prop.read_ty.unwrap());
                unsafe {
                    core::ptr::write(
                        props_data.add(idx),
                        AstTableProp {
                            name: name_ast,
                            location: Location::default(),
                            r#type: read_ty_rehydrated,
                            access: AstTableAccess::ReadWrite,
                            access_location: None,
                        },
                    );
                }
                idx += 1;
            } else {
                if let Some(read_ty) = prop.read_ty {
                    let read_ty_rehydrated = self.visit_type(read_ty);
                    unsafe {
                        core::ptr::write(
                            props_data.add(idx),
                            AstTableProp {
                                name: name_ast,
                                location: Location::default(),
                                r#type: read_ty_rehydrated,
                                access: AstTableAccess::Read,
                                access_location: None,
                            },
                        );
                    }
                    idx += 1;
                }

                if let Some(write_ty) = prop.write_ty {
                    let write_ty_rehydrated = self.visit_type(write_ty);
                    unsafe {
                        core::ptr::write(
                            props_data.add(idx),
                            AstTableProp {
                                name: name_ast,
                                location: Location::default(),
                                r#type: write_ty_rehydrated,
                                access: AstTableAccess::Write,
                                access_location: None,
                            },
                        );
                    }
                    idx += 1;
                }
            }
        }

        let indexer = if let Some(ref indexer_ref) = ttv.indexer {
            let _counter_indexer = RecursionCounter::recursion_counter_i32(
                &mut self.count as *mut i32 as *mut core::ffi::c_int,
            );

            let index_type = self.visit_type(indexer_ref.index_type);
            let result_type = self.visit_type(indexer_ref.index_result_type);

            let indexer_node = AstTableIndexer {
                index_type,
                result_type,
                location: Location::default(),
                access: AstTableAccess::ReadWrite,
                access_location: None,
            };

            let allocator = unsafe { &mut *self.allocator };
            allocator.alloc(indexer_node)
        } else {
            core::ptr::null_mut()
        };

        let props_array = AstArray {
            data: props_data,
            size: props_size,
        };

        let table_node = AstTypeTable::new(Location::default(), props_array, indexer);

        let allocator = unsafe { &mut *self.allocator };
        allocator.alloc(table_node) as *mut AstType
    }
}
