use super::*;

#[derive(Copy, Clone)]
pub struct SkipMany1<P>(P);

impl<P> ParserOnce for SkipMany1<P>
where
    P: ParserMut,
{
    type Input = P::Input;
    type Output = ();

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P> ParserMut for SkipMany1<P>
where
    P: ParserMut,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .many1_mut(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P> Parser for SkipMany1<P>
where
    P: Parser,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_ref()
            .many1(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse(input)
    }
}

pub fn skip_many1<P>(parser: P) -> SkipMany1<P> {
    SkipMany1(parser)
}
