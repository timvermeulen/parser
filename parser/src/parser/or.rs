use super::*;

#[derive(Copy, Clone)]
pub struct Or<P, Q>(P, Q);

impl<P, Q, I> ParserOnce<I> for Or<P, Q>
where
    P: ParserOnce<I>,
    Q: ParserOnce<I, Output = P::Output>,
    I: Stream,
{
    type Output = P::Output;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        match self.0.parse_once_and_check_consumed(input) {
            (None, false) => self.1.parse_once(input),
            (result, _) => result,
        }
    }
}

impl<P, Q, I> ParserMut<I> for Or<P, Q>
where
    P: ParserMut<I>,
    Q: ParserMut<I, Output = P::Output>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        match self.0.parse_mut_and_check_consumed(input) {
            (None, false) => self.1.parse_mut(input),
            (result, _) => result,
        }
    }
}

impl<P, Q, I> Parser<I> for Or<P, Q>
where
    P: Parser<I>,
    Q: Parser<I, Output = P::Output>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        match self.0.parse_and_check_consumed(input) {
            (None, false) => self.1.parse(input),
            (result, _) => result,
        }
    }
}

pub fn or<P, Q>(p: P, q: Q) -> Or<P, Q> {
    Or(p, q)
}
