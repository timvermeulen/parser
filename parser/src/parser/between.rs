use super::*;

#[derive(Copy, Clone)]
pub struct Between<P, L, R> {
    parser: P,
    left: L,
    right: R,
}

impl<P, L, R> ParserOnce for Between<P, L, R>
where
    P: ParserOnce,
    L: ParserOnce<Input = P::Input>,
    R: ParserOnce<Input = P::Input>,
{
    type Input = P::Input;
    type Output = P::Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        chain((self.left, self.parser, self.right))
            .map(|(_, output, _)| output)
            .parse_once(input)
    }
}

impl<P, L, R> ParserMut for Between<P, L, R>
where
    P: ParserMut,
    L: ParserMut<Input = P::Input>,
    R: ParserMut<Input = P::Input>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        chain((&mut self.left, &mut self.parser, &mut self.right))
            .map(|(_, output, _)| output)
            .parse_mut(input)
    }
}

impl<P, L, R> Parser for Between<P, L, R>
where
    P: Parser,
    L: Parser<Input = P::Input>,
    R: Parser<Input = P::Input>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        chain((&self.left, &self.parser, &self.right))
            .map(|(_, output, _)| output)
            .parse(input)
    }
}

pub fn between<P, L, R>(parser: P, left: L, right: R) -> Between<P, L, R> {
    Between {
        parser,
        left,
        right,
    }
}
