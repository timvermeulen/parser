use super::*;
use std::ops::Try;

pub struct Iter<P, I, O> {
    parser: P,
    input: I,
    first: Option<O>,
}

impl<P> Iterator for Iter<P, &mut P::Input, P::Output>
where
    P: ParserMut,
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
        R: Try<Ok = B>,
    {
        if let Some(first) = self.first.take() {
            state = f(state, first)?;
        }

        while let Some(output) = self.parser.parse_mut(self.input) {
            state = f(state, output)?
        }

        Try::from_ok(state)
    }
}

#[derive(Copy, Clone)]
pub struct Many1Mut<P, F> {
    parser: P,
    f: F,
}

impl<P, F, O> ParserOnce for Many1Mut<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<&mut P, &mut P::Input, P::Output>) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, O> ParserMut for Many1Mut<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<&mut P, &mut P::Input, P::Output>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let mut iter = Iter {
            parser: &mut self.parser,
            input,
            first: Some(first),
        };

        (self.f)(&mut iter)
    }
}

pub fn many1_mut<P, F>(parser: P, f: F) -> Many1Mut<P, F> {
    Many1Mut { parser, f }
}

#[derive(Copy, Clone)]
pub struct Many1<P, F> {
    parser: P,
    f: F,
}

impl<P, F, O> ParserOnce for Many1<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<&P, &mut P::Input, P::Output>) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, O> ParserMut for Many1<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<&P, &mut P::Input, P::Output>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        let first = self.parser.parse_mut(input)?;

        let mut iter = Iter {
            parser: &self.parser,
            input,
            first: Some(first),
        };

        (self.f)(&mut iter)
    }
}

impl<P, F, O> Parser for Many1<P, F>
where
    P: Parser,
    F: Fn(&mut Iter<&P, &mut P::Input, P::Output>) -> Option<O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        let first = self.parser.parse(input)?;

        let mut iter = Iter {
            parser: &self.parser,
            input,
            first: Some(first),
        };

        (self.f)(&mut iter)
    }
}

pub fn many1<P, F>(parser: P, f: F) -> Many1<P, F> {
    Many1 { parser, f }
}
