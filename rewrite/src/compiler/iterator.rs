use crate::compiler::nesting::Context;

pub struct ContextIterator<'a> {
    ctx: &'a [Context],
    index: usize,
}

impl<'a> ContextIterator<'a> {
    pub fn new(ctx: &'a [Context], index: usize) -> Self {
        Self { ctx, index }
    }

    pub fn next(&mut self) -> Option<&'a Context> {
        let context = self.ctx.get(self.index)?;
        self.index += 1;
        Some(context)
    }

    pub fn consume(&mut self) {
        self.index += 1;
    }

    pub fn def(&mut self, index: usize) {
        self.index = index;
    }

    pub fn peek(&self) -> Option<&'a Context> {
        self.ctx.get(self.index)
    }

    pub fn back(&mut self) -> Option<&'a Context> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;
        self.ctx.get(self.index)
    }

    pub fn position(&self) -> usize {
        self.index
    }

    pub fn set_position(&mut self, index: usize) {
        self.index = index.min(self.ctx.len());
    }

    pub fn remaining(&self) -> usize {
        self.ctx.len() - self.index
    }
}
