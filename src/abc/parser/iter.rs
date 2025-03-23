use super::{opmatch::OpMatch, Instruction, Op, OpCode};

#[derive(Debug)]
pub struct InsIterator<'a> {
    pub(crate) instructions: &'a [Instruction],
    pub(crate) cursor: usize,
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
    pub fn prev(&self) -> Option<&'a Instruction> {
        if self.cursor > 0 {
            Some(&self.instructions[self.cursor - 1])
        } else {
            None
        }
    }
    pub fn prev_op(&mut self) -> Option<&'a Op> {
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
    pub fn skip_until<M: OpMatch>(&mut self, matcher: &M) -> Option<&mut Self> {
        while !self.is(matcher) {
            self.next()?;
        }
        Some(self)
    }
    pub fn skip_until_match<M: OpMatch>(&'_ mut self, matcher: &M) -> Vec<&'a Op> {
        loop {
            if let Some(length) = matcher.matches(self) {
                self.cursor += length;
                return self.instructions[self.cursor - length..self.cursor]
                    .iter()
                    .map(|ins| &ins.op)
                    .collect();
            }
            if self.next().is_none() {
                break;
            }
        }
        Vec::new()
    }
    pub fn skip_until_op<M: OpMatch>(&'_ mut self, matcher: &M) -> Option<&'a Op> {
        self.skip_until(matcher).map(|s| &s.get().op)
    }
    pub fn next_op(&mut self) -> Option<&'a Op> {
        self.next().map(|p| &p.get().op)
    }

    pub fn is<M: OpMatch>(&self, matcher: &M) -> bool {
        matcher.matches(self).is_some()
    }
    pub fn is_next<M: OpMatch>(&self, matcher: &M) -> bool {
        let prog = Self {
            instructions: self.instructions,
            cursor: self.cursor + 1,
        };
        self.has_next() && matcher.matches(&prog).is_some()
    }

    pub fn get(&'_ self) -> &'a Instruction {
        &self.instructions[self.cursor]
    }
    pub fn get_match<M: OpMatch>(&'_ mut self, matcher: &M) -> Vec<&'a Op> {
        match matcher.matches(self) {
            Some(length) => {
                self.cursor += length;
                self.instructions[self.cursor - length..self.cursor]
                    .iter()
                    .map(|ins| &ins.op)
                    .collect()
            }
            None => Vec::new(),
        }
    }

    pub fn peek(&self, offset: i32) -> Option<&'a Op> {
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
