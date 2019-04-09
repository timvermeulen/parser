use super::*;

#[derive(Copy, Clone)]
pub struct AndThen<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for AndThen<P, F>
where
    P: ParserOnce<I>,
    F: FnOnce(P::Output) -> Option<O>,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_once(input).and_then(self.f)
    }
}

impl<P, F, I, O> ParserMut<I> for AndThen<P, F>
where
    P: ParserMut<I>,
    F: FnMut(P::Output) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_mut(input).and_then(&mut self.f)
    }
}

impl<P, F, I, O> Parser<I> for AndThen<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> Option<O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse(input).and_then(&self.f)
    }
}

pub fn and_then<P, F>(parser: P, f: F) -> AndThen<P, F> {
    AndThen { parser, f }
}
