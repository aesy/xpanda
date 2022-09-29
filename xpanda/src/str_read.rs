use crate::forward_peekable::{ForwardPeekable, IteratorExt};
use std::str::CharIndices;

pub struct StrRead<'a> {
    index: usize,
    line: usize,
    col: usize,
    input: &'a str,
    iter: ForwardPeekable<CharIndices<'a>>,
}

impl<'a> StrRead<'a> {
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            index: 0,
            line: 1,
            col: 1,
            input,
            iter: input.char_indices().forward_peekable(),
        }
    }

    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn col(&self) -> usize {
        self.col
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.iter.peek().map(|(_, c)| *c)
    }

    pub fn consume_char(&mut self) -> Option<char> {
        let (i, c) = self.iter.next()?;

        self.index = i + c.len_utf8();

        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(c)
    }

    pub fn consume_while<P>(&mut self, mut predicate: P) -> &'a str
    where
        P: FnMut(char) -> bool,
    {
        let start = self.index;

        while let Some(c) = self.peek_char() {
            if !predicate(c) {
                break;
            }

            self.consume_char();
        }

        let end = self.index;

        &self.input[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_char() {
        let mut reader = StrRead::new("hi");
        assert_eq!(reader.peek_char(), Some('h'));
        assert_eq!(reader.peek_char(), Some('h'));
    }

    #[test]
    fn consume_while() {
        let mut reader = StrRead::new("hi!");
        assert_eq!(reader.consume_while(char::is_alphabetic), "hi");
        assert_eq!(reader.consume_while(|c| true), "!");
        assert_eq!(reader.consume_while(|c| true), "");
    }
}
