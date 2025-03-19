use super::{Instruction, Op, OpCode};

#[derive(Debug)]
pub struct InsIterator<'a> {
    instructions: &'a [Instruction],
    cursor: usize,
}

impl<'a> InsIterator<'a> {
    fn sanity(&mut self) -> Option<&mut Self> {
        if self.has_next() {
            Some(self)
        } else {
            None
        }
    }
    pub fn new(instructions: &'a [Instruction], cursor: usize) -> Option<Self> {
        let prog = Self {
            instructions,
            cursor,
        };
        if prog.has_next() {
            Some(prog)
        } else {
            None
        }
    }

    pub fn is_jump(&self) -> bool {
        self.get().is_jump()
    }
    pub fn opcode(&self) -> OpCode {
        self.get().opcode
    }

    /// Get the previous instruction
    pub fn prev(&self) -> Option<&Instruction> {
        if self.cursor > 0 {
            Some(&self.instructions[self.cursor - 1])
        } else {
            None
        }
    }
    pub fn prev_op(&mut self) -> Option<&Op> {
        self.prev().map(|p| &p.op)
    }
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut Self> {
        self.cursor += 1;
        self.sanity()
    }
    pub fn has_next(&self) -> bool {
        self.cursor < self.instructions.len()
    }
    pub fn skip_until(&mut self, opcode: OpCode) -> Option<&mut Self> {
        while !self.is(opcode) {
            self.next()?;
        }
        Some(self)
    }
    pub fn skip_until_seq(&mut self, seq: &[OpCode]) -> Option<&Self> {
        while !self.is_sequence(seq) {
            self.next()?;
        }
        Some(self)
    }
    pub fn next_op(&mut self) -> Option<&Op> {
        self.next().map(|p| &p.get().op)
    }
    pub fn skip_until_op(&'_ mut self, opcode: OpCode) -> Option<&'a Op> {
        self.skip_until(opcode).map(|s| &s.get().op)
    }

    pub fn is(&self, opcode: OpCode) -> bool {
        self.get().opcode == opcode
    }
    pub fn is_next(&self, opcode: OpCode) -> bool {
        self.has_next() && self.instructions[self.cursor + 1].opcode == opcode
    }
    pub fn is_sequence(&self, seq: &[OpCode]) -> bool {
        if self.cursor + seq.len() > self.instructions.len() {
            return false;
        }
        for (i, opcode) in seq.iter().enumerate() {
            if self.instructions[self.cursor + i].opcode != *opcode {
                return false;
            }
        }
        true
    }

    pub fn get(&'_ self) -> &'a Instruction {
        &self.instructions[self.cursor]
    }
    pub fn get_sequence(&'_ mut self, seq: &[OpCode]) -> Vec<&Op> {
        if self.is_sequence(seq) {
            self.cursor += seq.len();
            self.instructions[self.cursor - seq.len()..self.cursor]
                .iter()
                .map(|ins| &ins.op)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn peek(&self, offset: i32) -> Option<&Op> {
        self.instructions
            .get((self.cursor as i32 + offset) as usize)
            .map(|ins| &ins.op)
    }

    pub fn tell(&self) -> usize {
        self.cursor
    }

    // pub fn get_mut(&mut self) -> &mut Instruction {
    //     &mut self.instructions[self.cursor]
    // }
}

pub trait InsIter {
    fn iter_prog(&self) -> InsIterator;
}

impl InsIter for Vec<Instruction> {
    fn iter_prog(&self) -> InsIterator {
        InsIterator {
            instructions: self,
            cursor: 0,
        }
    }
}
impl InsIter for &[Instruction] {
    fn iter_prog(&self) -> InsIterator {
        InsIterator {
            instructions: self,
            cursor: 0,
        }
    }
}
impl InsIter for Option<&[Instruction]> {
    fn iter_prog(&self) -> InsIterator {
        InsIterator {
            instructions: self.unwrap_or(&[]),
            cursor: 0,
        }
    }
}
