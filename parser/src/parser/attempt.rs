use super::*;

#[derive(Copy, Clone)]
pub struct Attempt<P>(P);

impl<P> ParserOnce for Attempt<P>
where
    P: ParserOnce,
{
    type Input = P::Input;
    type Output = P::Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        let copy = *input;
        let output = self.0.parse_once(input);
        if output.is_none() {
            *input = copy;
        }
        output
    }
}

impl<P> ParserMut for Attempt<P>
where
    P: ParserMut,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        let copy = *input;
        let output = self.0.parse_mut(input);
        if output.is_none() {
            *input = copy;
        }
        output
    }
}

impl<P> Parser for Attempt<P>
where
    P: Parser,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        let copy = *input;
        let output = self.0.parse(input);
        if output.is_none() {
            *input = copy;
        }
        output
    }
}

pub fn attempt<P>(parser: P) -> Attempt<P> {
    Attempt(parser)
}
