//! Source: `Analysis/include/Luau/ControlFlowGraph.h:306-312` (hand-ported)
//! C++ `template<typename T, typename... Args> NotNull<T> CFGBuilder::emit(Block* block, Args&&... args)`.
use crate::records::assign::Assign;
use crate::records::block::Block;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::declare::Declare;
use crate::records::join::Join;
use crate::records::refine::Refine;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use crate::type_aliases::instruction::{Instruction, InstructionMember};
use crate::type_aliases::refinement_control_flow_graph::Refinement;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_local::AstStatLocal;

/// Bridges the C++ `T{args...}` variadic construction: maps each instruction
/// type `T` and its constructor arg-pack to a built `Instruction` variant.
/// (C++ `instructions.allocate(T{std::forward<Args>(args)...})`.)
pub trait IntoInstruction<T> {
    fn into_instruction(self) -> Instruction;
}

// emit<Declare>(block, (def, source)) -> Declare(def, source)
impl IntoInstruction<Declare> for (DefId, *mut AstStatLocal) {
    fn into_instruction(self) -> Instruction {
        Instruction::Declare(Declare::declare(self.0, self.1))
    }
}

// emit<Assign>(block, (def, source)) -> Assign(def, source)
impl IntoInstruction<Assign> for (DefId, *mut AstStatAssign) {
    fn into_instruction(self) -> Instruction {
        Instruction::Assign(Assign::assign(self.0, self.1))
    }
}

// emit<Join>(block, def) -> Join(definition)
impl IntoInstruction<Join> for DefId {
    fn into_instruction(self) -> Instruction {
        // C++ `explicit Join(DefId definition)` — operands start empty.
        Instruction::Join(Join {
            definition: self,
            operands: alloc::vec::Vec::new(),
        })
    }
}

// emit<Refine>(block, (definition, prop)) -> Refine(definition, prop)
impl IntoInstruction<Refine> for (DefId, *const Refinement) {
    fn into_instruction(self) -> Instruction {
        Instruction::Refine(Refine {
            definition: self.0,
            prop: self.1,
        })
    }
}

impl CfgBuilder {
    /// `template<typename T, typename... Args> NotNull<T> emit(Block* block, Args&&...)`.
    /// `NotNull<T>` -> `*mut T`.
    pub fn emit<T, Args>(&mut self, block: *mut Block, args: Args) -> *mut T
    where
        Args: IntoInstruction<T>,
        T: InstructionMember,
    {
        // C++:
        //   InstrId inst = allocator->newInstruction<T>(std::forward<Args>(args)...);
        //   block->instructions.emplace_back(inst);
        //   return NotNull{inst->template get_if<T>()};
        let allocator = unsafe { &mut *self.allocator };
        let inst = allocator.new_instruction(args.into_instruction());
        unsafe {
            (*block).instructions.push(inst);
            <T as InstructionMember>::get_if_mut(&mut *inst).unwrap() as *mut T
        }
    }
}
