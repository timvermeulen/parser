use super::*;

#[derive(Copy, Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<P, F, O> ParserOnce for Map<P, F>
where
    P: ParserOnce,
    F: FnOnce(P::Output) -> O,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_once(input).map(self.f)
    }
}

impl<P, F, O> ParserMut for Map<P, F>
where
    P: ParserMut,
    F: FnMut(P::Output) -> O,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_mut(input).map(&mut self.f)
    }
}

impl<P, F, O> Parser for Map<P, F>
where
    P: Parser,
    F: Fn(P::Output) -> O,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse(input).map(&self.f)
    }
}

pub fn map<P, F>(parser: P, f: F) -> Map<P, F> {
    Map { parser, f }
}
