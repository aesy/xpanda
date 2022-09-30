use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct ForwardPeekable<I>
where
    I: Iterator,
{
    iter: I,
    peeked: VecDeque<I::Item>,
}

impl<I> ForwardPeekable<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeked: VecDeque::new(),
        }
    }
}

impl<I> ForwardPeekable<I>
where
    I: Iterator,
{
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        let len = self.peeked.len();

        for _ in len..=n {
            let e = self.iter.next()?;
            self.peeked.push_back(e);
        }

        self.peeked.get(n)
    }
}

impl<I> Iterator for ForwardPeekable<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.peeked.pop_front().or_else(|| self.iter.next())
    }
}

pub trait IteratorExt: Iterator {
    fn forward_peekable(self) -> ForwardPeekable<Self>
    where
        Self: Sized,
    {
        ForwardPeekable::new(self)
    }
}

impl<T> IteratorExt for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek() {
        let mut peekable = [1, 2, 3].into_iter().forward_peekable();

        // peek one element ahead
        assert_eq!(peekable.peek(), Some(&1));

        // peeking does not advance the iterator
        assert_eq!(peekable.next(), Some(1));

        // we can peek multiple times
        assert_eq!(peekable.peek(), Some(&2));
        assert_eq!(peekable.peek(), Some(&2));

        // after the iterator is finished, so is peek
        peekable.next();
        peekable.next();
        assert_eq!(peekable.peek(), None);
    }

    #[test]
    fn peek_nth() {
        let mut peekable = [1, 2, 3].into_iter().forward_peekable();

        // peek ahead
        assert_eq!(peekable.peek_nth(0), Some(&1));
        assert_eq!(peekable.peek_nth(1), Some(&2));

        // peek() always return next value
        assert_eq!(peekable.peek(), Some(&1));
        assert_eq!(peekable.peek(), Some(&1));

        // peeking does not advance the iterator
        assert_eq!(peekable.next(), Some(1));

        // elements may be peeked multiple times
        assert_eq!(peekable.peek_nth(0), Some(&2));
        assert_eq!(peekable.peek_nth(0), Some(&2));

        // there is nothing past the end
        assert_eq!(peekable.peek_nth(2), None);
    }
}
