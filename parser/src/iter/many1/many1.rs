use super::*;
use std::ops::Try;

pub struct Iter<'a, P, I>
where
    P: ParserMut<I>,
{
    parser: &'a P,
    input: &'a mut I,
    first: Option<P::Output>,
}

impl<P, I> Iterator for Iter<'_, P, I>
where
    P: Parser<I>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.first.take().or_else(|| self.parser.parse(self.input))
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
pub struct Many1<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for Many1<P, F>
where
    P: ParserMut<I>,
    F: FnMut(Iter<'_, P, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, I, O> ParserMut<I> for Many1<P, F>
where
    P: ParserMut<I>,
    F: FnMut(Iter<'_, P, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let iter = Iter {
            parser: &self.parser,
            input,
            first: Some(first),
        };

        (self.f)(iter)
    }
}

impl<P, F, I, O> Parser<I> for Many1<P, F>
where
    P: Parser<I>,
    F: Fn(Iter<'_, P, I>) -> Option<O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse(input)?;

        let iter = Iter {
            parser: &self.parser,
            input,
            first: Some(first),
        };

        (self.f)(iter)
    }
}

pub fn many1<P, F>(parser: P, f: F) -> Many1<P, F> {
    Many1 { parser, f }
}
