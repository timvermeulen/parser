use super::*;

#[derive(Copy, Clone)]
pub struct SkipMany<P>(P);

impl<P> ParserOnce for SkipMany<P>
where
    P: ParserMut,
{
    type Input = P::Input;
    type Output = ();

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P> ParserMut for SkipMany<P>
where
    P: ParserMut,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .many_mut(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P> Parser for SkipMany<P>
where
    P: Parser,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_ref()
            .many(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse(input)
    }
}

pub fn skip_many<P>(parser: P) -> SkipMany<P> {
    SkipMany(parser)
}
