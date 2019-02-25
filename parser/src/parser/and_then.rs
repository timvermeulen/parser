use super::*;

#[derive(Copy, Clone)]
pub struct AndThen<P, F> {
    parser: P,
    f: F,
}

impl<P, F, O> ParserOnce for AndThen<P, F>
where
    P: ParserOnce,
    F: FnOnce(P::Output) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_once(input).and_then(self.f)
    }
}

impl<P, F, O> ParserMut for AndThen<P, F>
where
    P: ParserMut,
    F: FnMut(P::Output) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_mut(input).and_then(&mut self.f)
    }
}

impl<P, F, O> Parser for AndThen<P, F>
where
    P: Parser,
    F: Fn(P::Output) -> Option<O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse(input).and_then(&self.f)
    }
}

pub fn and_then<P, F>(parser: P, f: F) -> AndThen<P, F> {
    AndThen { parser, f }
}
