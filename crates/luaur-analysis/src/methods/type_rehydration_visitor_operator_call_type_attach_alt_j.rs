use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::get_type_alt_j::get as get_type;
use crate::functions::get_type_pack::get as get_type_pack;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::recursion_counter::RecursionCounter;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_generic_type::AstGenericType;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;
use luaur_ast::type_aliases::ast_argument_name::AstArgumentName;

impl TypeRehydrationVisitor {
    pub fn operator_call_6(&mut self, ftv: &FunctionType) -> *mut AstType {
        let _recursion_counter = RecursionCounter::recursion_counter_i32(
            &mut self.count as *mut i32 as *mut core::ffi::c_int,
        );

        if self.has_seen(ftv as *const FunctionType as *const core::ffi::c_void) {
            let cycle_ref = AstTypeReference::new(
                Location::default(),
                None,
                AstName::ast_name_c_char(c"<Cycle>".as_ptr() as *const core::ffi::c_char),
                None,
                Location::default(),
                false,
                AstArray::default(),
            );
            let allocator = unsafe { &mut *self.allocator };
            return allocator.alloc(cycle_ref) as *mut AstType;
        }

        // generics
        let mut generics_array = AstArray::<*mut AstGenericType> {
            data: core::ptr::null_mut(),
            size: 0,
        };
        if !ftv.generics.is_empty() {
            let generics_ptr = unsafe {
                (*self.allocator)
                    .allocate(core::mem::size_of::<*mut AstGenericType>() * ftv.generics.len())
                    as *mut *mut AstGenericType
            };
            generics_array.data = generics_ptr;
            let mut num_generics: usize = 0;
            for &gen_id in &ftv.generics {
                let gen_ptr = unsafe { get_type::<GenericType>(gen_id) };
                if !gen_ptr.is_null() {
                    let gen = unsafe { &*gen_ptr };
                    let ast_gen = AstGenericType::new(
                        Location::default(),
                        AstName::ast_name_c_char(gen.name.as_ptr() as *const core::ffi::c_char),
                        core::ptr::null_mut(),
                    );
                    let allocator = unsafe { &mut *self.allocator };
                    unsafe { *generics_ptr.add(num_generics) = allocator.alloc(ast_gen) };
                    num_generics += 1;
                }
            }
            generics_array.size = num_generics;
        }

        // generic packs
        let mut generic_packs_array = AstArray::<*mut AstGenericTypePack> {
            data: core::ptr::null_mut(),
            size: 0,
        };
        if !ftv.generic_packs.is_empty() {
            let packs_ptr = unsafe {
                (*self.allocator).allocate(
                    core::mem::size_of::<*mut AstGenericTypePack>() * ftv.generic_packs.len(),
                ) as *mut *mut AstGenericTypePack
            };
            generic_packs_array.data = packs_ptr;
            let mut num_packs: usize = 0;
            for &pack_id in &ftv.generic_packs {
                let pack_ptr = unsafe { get_type_pack::<GenericTypePack>(pack_id) };
                if !pack_ptr.is_null() {
                    let pack = unsafe { &*pack_ptr };
                    let ast_pack = AstGenericTypePack::new(
                        Location::default(),
                        AstName::ast_name_c_char(pack.name.as_ptr() as *const core::ffi::c_char),
                        core::ptr::null_mut(),
                    );
                    let allocator = unsafe { &mut *self.allocator };
                    unsafe { *packs_ptr.add(num_packs) = allocator.alloc(ast_pack) };
                    num_packs += 1;
                }
            }
            generic_packs_array.size = num_packs;
        }

        // argument types
        let (arg_vector, arg_tail) = flatten_type_pack_id(ftv.arg_types);
        let arg_size = arg_vector.len();
        let arg_data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<*mut AstType>() * arg_size)
                as *mut *mut AstType
        };
        for i in 0..arg_size {
            let _counter = RecursionCounter::recursion_counter_i32(
                &mut self.count as *mut i32 as *mut core::ffi::c_int,
            );
            let rehydrated = self.visit_type(arg_vector[i]);
            unsafe { *arg_data.add(i) = rehydrated };
        }
        let arg_types_array = AstArray::<*mut AstType> {
            data: arg_data,
            size: arg_size,
        };

        let arg_tail_annotation = if let Some(tp) = arg_tail {
            self.rehydrate(tp)
        } else {
            core::ptr::null_mut()
        };

        // argument names
        let mut arg_names_array = AstArray::<Option<AstArgumentName>> {
            data: core::ptr::null_mut(),
            size: 0,
        };
        if !ftv.arg_names.is_empty() {
            let names_ptr = unsafe {
                (*self.allocator)
                    .allocate(core::mem::size_of::<Option<AstArgumentName>>() * ftv.arg_names.len())
                    as *mut Option<AstArgumentName>
            };
            let mut i: usize = 0;
            for arg_opt in &ftv.arg_names {
                let slot: Option<AstArgumentName> = if let Some(ref arg) = *arg_opt {
                    let name =
                        AstName::ast_name_c_char(arg.name.as_ptr() as *const core::ffi::c_char);
                    Some((name, Location::default()))
                } else {
                    None
                };
                unsafe { core::ptr::write(names_ptr.add(i), slot) };
                i += 1;
            }
            arg_names_array.data = names_ptr;
            arg_names_array.size = i;
        }

        // return types
        let (ret_vector, ret_tail) = flatten_type_pack_id(ftv.ret_types);
        let ret_size = ret_vector.len();
        let ret_data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<*mut AstType>() * ret_size)
                as *mut *mut AstType
        };
        for i in 0..ret_size {
            let _counter = RecursionCounter::recursion_counter_i32(
                &mut self.count as *mut i32 as *mut core::ffi::c_int,
            );
            let rehydrated = self.visit_type(ret_vector[i]);
            unsafe { *ret_data.add(i) = rehydrated };
        }
        let return_types_array = AstArray::<*mut AstType> {
            data: ret_data,
            size: ret_size,
        };

        let ret_tail_annotation = if let Some(tp) = ret_tail {
            self.rehydrate(tp)
        } else {
            core::ptr::null_mut()
        };

        let allocator = unsafe { &mut *self.allocator };
        let return_annotation = allocator.alloc(AstTypePackExplicit::new(
            Location::default(),
            AstTypeList {
                types: return_types_array,
                tail_type: ret_tail_annotation,
            },
        )) as *mut AstTypePack;

        let func_type = AstTypeFunction::ast_type_function_location_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
            Location::default(),
            generics_array,
            generic_packs_array,
            AstTypeList {
                types: arg_types_array,
                tail_type: arg_tail_annotation,
            },
            arg_names_array,
            return_annotation,
        );

        allocator.alloc(func_type) as *mut AstType
    }
}
