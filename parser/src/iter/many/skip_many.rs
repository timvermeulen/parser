use super::*;

#[derive(Copy, Clone)]
pub struct SkipMany<P>(P);

impl<P, I> ParserOnce<I> for SkipMany<P>
where
    P: ParserMut<I>,
{
    type Output = ();

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I> ParserMut<I> for SkipMany<P>
where
    P: ParserMut<I>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .many_mut(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P, I> Parser<I> for SkipMany<P>
where
    P: Parser<I>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
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
