use super::*;

#[derive(Copy, Clone)]
pub struct FlatMap<P, F> {
    parser: P,
    f: F,
}

impl<P, Q, F> ParserOnce for FlatMap<P, F>
where
    P: ParserOnce,
    Q: ParserOnce<Input = P::Input>,
    F: FnOnce(P::Output) -> Q,
{
    type Input = P::Input;
    type Output = Q::Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        let f = self.f;
        self.parser
            .parse_once(input)
            .and_then(|o| f(o).parse_once(input))
    }
}

impl<P, Q, F> ParserMut for FlatMap<P, F>
where
    P: ParserMut,
    Q: ParserOnce<Input = P::Input>,
    F: FnMut(P::Output) -> Q,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .parse_mut(input)
            .and_then(|o| (self.f)(o).parse_once(input))
    }
}

impl<P, Q, F> Parser for FlatMap<P, F>
where
    P: Parser,
    Q: ParserOnce<Input = P::Input>,
    F: Fn(P::Output) -> Q,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .parse(input)
            .and_then(|o| (self.f)(o).parse_once(input))
    }
}

pub fn flat_map<P, F>(parser: P, f: F) -> FlatMap<P, F> {
    FlatMap { parser, f }
}
