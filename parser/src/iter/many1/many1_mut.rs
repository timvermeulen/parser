use super::*;
use std::ops::Try;

pub struct IterMut<'a, P, I>
where
    P: ParserOnce<I>,
{
    parser: &'a mut P,
    input: &'a mut I,
    first: Option<P::Output>,
}

impl<P, I> Iterator for IterMut<'_, P, I>
where
    P: ParserMut<I>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.first
            .take()
            .or_else(|| self.parser.parse_mut(self.input))
    }

    fn try_fold<B, F, R>(&mut self, mut state: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        if let Some(first) = self.first.take() {
            state = f(state, first)?;
        }

        while let Some(output) = self.parser.parse_mut(self.input) {
            state = f(state, output)?
        }

        Try::from_output(state)
    }
}

#[derive(Copy, Clone)]
pub struct Many1Mut<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for Many1Mut<P, F>
where
    P: ParserMut<I>,
    F: FnOnce(IterMut<'_, P, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let iter = IterMut {
            parser: &mut self.parser,
            input,
            first: Some(first),
        };

        (self.f)(iter)
    }
}

impl<P, F, I, O> ParserMut<I> for Many1Mut<P, F>
where
    P: ParserMut<I>,
    F: FnMut(IterMut<'_, P, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let iter = IterMut {
            parser: &mut self.parser,
            input,
            first: Some(first),
        };

        (self.f)(iter)
    }
}

pub fn many1_mut<P, F>(parser: P, f: F) -> Many1Mut<P, F> {
    Many1Mut { parser, f }
}
