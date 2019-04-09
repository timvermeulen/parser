use super::*;

#[derive(Copy, Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for Map<P, F>
where
    P: ParserOnce<I>,
    F: FnOnce(P::Output) -> O,
    I: Stream,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_once(input).map(self.f)
    }
}

impl<P, F, I, O> ParserMut<I> for Map<P, F>
where
    P: ParserMut<I>,
    F: FnMut(P::Output) -> O,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_mut(input).map(&mut self.f)
    }
}

impl<P, F, I, O> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> O,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse(input).map(&self.f)
    }
}

pub fn map<P, F>(parser: P, f: F) -> Map<P, F> {
    Map { parser, f }
}
