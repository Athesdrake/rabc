mod instruction;
mod iter;
mod opcodes;
mod parser;

pub mod opargs;

pub use instruction::{Instruction, Op};
pub use iter::{InsIter, InsIterator};
pub use opcodes::OpCode;
