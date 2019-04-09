use super::*;

#[derive(Copy, Clone)]
pub struct Attempt<P>(P);

impl<P, I> ParserOnce<I> for Attempt<P>
where
    P: ParserOnce<I>,
    I: Stream,
{
    type Output = P::Output;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        let copy = *input;
        let output = self.0.parse_once(input);
        if output.is_none() {
            *input = copy;
        }
        output
    }
}

impl<P, I> ParserMut<I> for Attempt<P>
where
    P: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let copy = *input;
        let output = self.0.parse_mut(input);
        if output.is_none() {
            *input = copy;
        }
        output
    }
}

impl<P, I> Parser<I> for Attempt<P>
where
    P: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
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
