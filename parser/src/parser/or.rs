use super::*;

pub struct Or<P, Q>(P, Q);

impl<P, Q> ParserOnce for Or<P, Q>
where
    P: ParserOnce,
    Q: ParserOnce<Input = P::Input, Output = P::Output>,
{
    type Input = P::Input;
    type Output = P::Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_once_and_check_consumed(input) {
            (None, false) => self.1.parse_once(input),
            (result, _) => result,
        }
    }
}

impl<P, Q> ParserMut for Or<P, Q>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input, Output = P::Output>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_mut_and_check_consumed(input) {
            (None, false) => self.1.parse_mut(input),
            (result, _) => result,
        }
    }
}

impl<P, Q> Parser for Or<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input, Output = P::Output>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_and_check_consumed(input) {
            (None, false) => self.1.parse(input),
            (result, _) => result,
        }
    }
}

pub fn or<P, Q>(p: P, q: Q) -> Or<P, Q> {
    Or(p, q)
}
