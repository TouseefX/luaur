use crate::functions::allocate_string_type_attach::allocate_string_luau_allocator_string_view;
use crate::records::extern_type::ExternType;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::enums::ast_table_access::AstTableAccess;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_table_indexer::AstTableIndexer;
use luaur_ast::records::ast_table_prop::AstTableProp;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_4(&mut self, etv: &ExternType) -> *mut AstType {
        let _counter = RecursionCounter::recursion_counter_i32(
            &mut self.count as *mut i32 as *mut core::ffi::c_int,
        );

        let name_ptr = {
            let allocator = unsafe { &mut *self.allocator };
            allocate_string_luau_allocator_string_view(allocator, &etv.name)
        };

        if !self.options.expand_extern_type_props
            || self.has_seen(etv as *const ExternType as *const core::ffi::c_void)
            || self.count > 1
        {
            let reference = AstTypeReference::new(
                Location::default(),
                None,
                AstName::ast_name_c_char(name_ptr),
                None,
                Location::default(),
                false,
                AstArray::default(),
            );
            return unsafe { (*self.allocator).alloc(reference) as *mut AstType };
        }

        let props_size = etv.props.len();
        let props_data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<AstTableProp>() * props_size)
                as *mut AstTableProp
        };

        let mut idx = 0;
        for (prop_name, prop) in &etv.props {
            let name = {
                let allocator = unsafe { &mut *self.allocator };
                allocate_string_luau_allocator_string_view(allocator, prop_name)
            };

            if prop.is_shared() {
                let read_type_ptr = self.visit_type(prop.read_ty.unwrap());
                unsafe {
                    props_data.add(idx).write(AstTableProp {
                        name: AstName::ast_name_c_char(name),
                        location: Location::default(),
                        r#type: read_type_ptr,
                        access: AstTableAccess::ReadWrite,
                        access_location: None,
                    });
                }
                idx += 1;
            } else {
                if let Some(read_ty) = prop.read_ty {
                    let read_type_ptr = self.visit_type(read_ty);
                    unsafe {
                        props_data.add(idx).write(AstTableProp {
                            name: AstName::ast_name_c_char(name),
                            location: Location::default(),
                            r#type: read_type_ptr,
                            access: AstTableAccess::Read,
                            access_location: None,
                        });
                    }
                    idx += 1;
                }

                if let Some(write_ty) = prop.write_ty {
                    let write_type_ptr = self.visit_type(write_ty);
                    unsafe {
                        props_data.add(idx).write(AstTableProp {
                            name: AstName::ast_name_c_char(name),
                            location: Location::default(),
                            r#type: write_type_ptr,
                            access: AstTableAccess::Write,
                            access_location: None,
                        });
                    }
                    idx += 1;
                }
            }
        }

        let props = AstArray {
            data: props_data,
            size: idx,
        };

        let indexer = if let Some(ref indexer_data) = etv.indexer {
            let _inner_counter = RecursionCounter::recursion_counter_i32(
                &mut self.count as *mut i32 as *mut core::ffi::c_int,
            );

            let index_type = self.visit_type(indexer_data.index_type);
            let result_type = self.visit_type(indexer_data.index_result_type);

            let allocator = unsafe { &mut *self.allocator };
            allocator.alloc(AstTableIndexer {
                index_type,
                result_type,
                location: Location::default(),
                access: AstTableAccess::ReadWrite,
                access_location: None,
            })
        } else {
            core::ptr::null_mut()
        };

        let table = AstTypeTable::new(Location::default(), props, indexer);
        let allocator = unsafe { &mut *self.allocator };
        allocator.alloc(table) as *mut AstType
    }
}
