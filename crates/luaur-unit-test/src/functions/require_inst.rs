use luaur_analysis::type_aliases::instruction::InstructionMember;

pub fn require_inst<T: InstructionMember>(
    block: *mut luaur_analysis::records::block::Block,
    idx: usize,
) -> *mut T {
    let instructions = unsafe { (*block).get_instructions() };
    assert!(idx < instructions.len());

    let inst = instructions[idx];
    let typed = T::get_if(unsafe { &*inst });
    assert!(typed.is_some());

    typed.unwrap() as *const T as *mut T
}
