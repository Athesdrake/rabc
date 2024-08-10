mod instruction;
mod iter;
mod opcodes;

pub mod opargs;

pub use instruction::{Instruction, Op};
pub use iter::{InsIter, InsIterator};
pub use opcodes::OpCode;
