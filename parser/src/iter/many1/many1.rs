use super::*;
use std::ops::Try;

pub struct Iter<P, I, O> {
    parser: *mut P,
    input: I,
    first: Option<O>,
}

impl<P, I> Iterator for Iter<P, &mut I, P::Output>
where
    P: ParserMut<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.first
            .take()
            .or_else(|| unsafe { (*self.parser).parse_mut(self.input) })
    }

    fn try_fold<B, F, R>(&mut self, mut state: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Ok = B>,
    {
        if let Some(first) = self.first.take() {
            state = f(state, first)?;
        }

        while let Some(output) = unsafe { (*self.parser).parse_mut(self.input) } {
            state = f(state, output)?
        }

        Try::from_ok(state)
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
    F: FnMut(&mut Iter<P, &mut I, P::Output>) -> Option<O>,
    I: Stream,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, I, O> ParserMut<I> for Many1<P, F>
where
    P: ParserMut<I>,
    F: FnMut(&mut Iter<P, &mut I, P::Output>) -> Option<O>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let mut iter = Iter {
            parser: &mut self.parser as *mut P,
            input,
            first: Some(first),
        };

        (self.f)(&mut iter)
    }
}

impl<P, F, I, O> Parser<I> for Many1<P, F>
where
    P: Parser<I>,
    F: Fn(&mut Iter<P, &mut I, P::Output>) -> Option<O>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let first = self.parser.parse(input)?;

        let mut iter = Iter {
            parser: &self.parser as *const P as *mut P,
            input,
            first: Some(first),
        };

        (self.f)(&mut iter)
    }
}

pub fn many1<P, F>(parser: P, f: F) -> Many1<P, F> {
    Many1 { parser, f }
}
