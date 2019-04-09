use super::*;

#[derive(Copy, Clone)]
pub struct SkipMany1<P>(P);

impl<P, I> ParserOnce<I> for SkipMany1<P>
where
    P: ParserMut<I>,
    I: Stream,
{
    type Output = ();

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I> ParserMut<I> for SkipMany1<P>
where
    P: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .many1_mut(|iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P, I> Parser<I> for SkipMany1<P>
where
    P: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
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
