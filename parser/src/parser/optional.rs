use super::*;

#[derive(Copy, Clone)]
pub struct Optional<P>(P);

impl<P> ParserOnce for Optional<P>
where
    P: ParserOnce,
{
    type Input = P::Input;
    type Output = Option<P::Output>;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
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

impl<P> ParserMut for Optional<P>
where
    P: ParserMut,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
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

impl<P> Parser for Optional<P>
where
    P: Parser,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
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
