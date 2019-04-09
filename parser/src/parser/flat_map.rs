use super::*;

#[derive(Copy, Clone)]
pub struct FlatMap<P, F> {
    parser: P,
    f: F,
}

impl<P, Q, F, I> ParserOnce<I> for FlatMap<P, F>
where
    P: ParserOnce<I>,
    Q: ParserOnce<I>,
    F: FnOnce(P::Output) -> Q,
    I: Stream,
{
    type Output = Q::Output;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        let f = self.f;
        self.parser
            .parse_once(input)
            .and_then(|o| f(o).parse_once(input))
    }
}

impl<P, Q, F, I> ParserMut<I> for FlatMap<P, F>
where
    P: ParserMut<I>,
    Q: ParserOnce<I>,
    F: FnMut(P::Output) -> Q,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .parse_mut(input)
            .and_then(|o| (self.f)(o).parse_once(input))
    }
}

impl<P, Q, F, I> Parser<I> for FlatMap<P, F>
where
    P: Parser<I>,
    Q: ParserOnce<I>,
    F: Fn(P::Output) -> Q,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .parse(input)
            .and_then(|o| (self.f)(o).parse_once(input))
    }
}

pub fn flat_map<P, F>(parser: P, f: F) -> FlatMap<P, F> {
    FlatMap { parser, f }
}
