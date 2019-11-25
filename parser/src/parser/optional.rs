use super::*;

#[derive(Copy, Clone)]
pub struct Optional<P>(P);

impl<P, I> ParserOnce<I> for Optional<P>
where
    P: ParserOnce<I>,
    I: Stream,
{
    type Output = Option<P::Output>;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        let (output, consumed) = self.0.parse_once_and_check_consumed(input);
        if output.is_some() {
            Some(output)
        } else if consumed {
            None
        } else {
            Some(None)
        }
    }
}

impl<P, I> ParserMut<I> for Optional<P>
where
    P: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let (output, consumed) = self.0.parse_mut_and_check_consumed(input);
        if output.is_some() {
            Some(output)
        } else if consumed {
            None
        } else {
            Some(None)
        }
    }
}

impl<P, I> Parser<I> for Optional<P>
where
    P: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let (output, consumed) = self.0.parse_and_check_consumed(input);
        if output.is_some() {
            Some(output)
        } else if consumed {
            None
        } else {
            Some(None)
        }
    }
}

pub fn optional<P>(p: P) -> Optional<P> {
    Optional(p)
}
