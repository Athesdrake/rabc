use std::ops::BitOr;

use super::{InsIterator, OpCode};

pub struct AnyOp(Vec<Box<dyn OpMatch>>);
#[derive(Debug, Clone, Copy)]
pub struct OpSeq<const S: usize>(pub [OpCode; S]);

pub trait OpMatch {
    fn matches(&self, prog: &InsIterator) -> Option<usize>;
}

impl OpMatch for OpCode {
    fn matches(&self, prog: &InsIterator) -> Option<usize> {
        (&prog.get().opcode == self).then_some(1)
    }
}

impl<const S: usize> OpMatch for OpSeq<S> {
    fn matches(&self, prog: &InsIterator) -> Option<usize> {
        self.0.matches(prog)
    }
}
impl<const S: usize> OpMatch for [OpCode; S] {
    fn matches(&self, prog: &InsIterator) -> Option<usize> {
        if prog.cursor + self.len() > prog.instructions.len() {
            return None;
        }
        for (i, opcode) in self.iter().enumerate() {
            if prog.instructions[prog.cursor + i].opcode != *opcode {
                return None;
            }
        }
        Some(self.len())
    }
}

impl OpMatch for AnyOp {
    fn matches(&self, prog: &InsIterator) -> Option<usize> {
        self.0.iter().find_map(|matcher| matcher.matches(prog))
    }
}

impl BitOr<OpCode> for OpCode {
    type Output = AnyOp;
    fn bitor(self, rhs: OpCode) -> Self::Output {
        AnyOp(vec![Box::new(self), Box::new(rhs)])
    }
}
impl BitOr<AnyOp> for OpCode {
    type Output = AnyOp;
    fn bitor(self, rhs: AnyOp) -> Self::Output {
        rhs | self
    }
}
impl<const S: usize> BitOr<OpSeq<S>> for OpCode {
    type Output = AnyOp;
    fn bitor(self, rhs: OpSeq<S>) -> Self::Output {
        rhs | self
    }
}

impl<const S: usize> BitOr<OpCode> for OpSeq<S> {
    type Output = AnyOp;
    fn bitor(self, rhs: OpCode) -> Self::Output {
        AnyOp(vec![Box::new(self), Box::new(rhs)])
    }
}
impl<const S: usize> BitOr<AnyOp> for OpSeq<S> {
    type Output = AnyOp;
    fn bitor(self, rhs: AnyOp) -> Self::Output {
        rhs | self
    }
}
impl<const S: usize, const T: usize> BitOr<OpSeq<S>> for OpSeq<T> {
    type Output = AnyOp;
    fn bitor(self, rhs: OpSeq<S>) -> Self::Output {
        AnyOp(vec![Box::new(self), Box::new(rhs)])
    }
}

impl BitOr<OpCode> for AnyOp {
    type Output = AnyOp;
    fn bitor(mut self, rhs: OpCode) -> Self::Output {
        self.0.push(Box::new(rhs));
        self
    }
}
impl<const S: usize> BitOr<OpSeq<S>> for AnyOp {
    type Output = AnyOp;
    fn bitor(mut self, rhs: OpSeq<S>) -> Self::Output {
        self.0.push(Box::new(rhs));
        self
    }
}
impl BitOr<AnyOp> for AnyOp {
    type Output = AnyOp;
    fn bitor(mut self, rhs: AnyOp) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}
