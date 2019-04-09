use super::*;

#[derive(Copy, Clone)]
pub struct SkipSepBy<P, Q> {
    parser: P,
    separator: Q,
}

impl<P, Q, I> ParserOnce<I> for SkipSepBy<P, Q>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
{
    type Output = ();

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, I> ParserMut<I> for SkipSepBy<P, Q>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .sep_by_mut(&mut self.separator, |iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P, Q, I> Parser<I> for SkipSepBy<P, Q>
where
    P: Parser<I>,
    Q: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_ref()
            .sep_by(&self.separator, |iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse(input)
    }
}

pub fn skip_sep_by<P, Q>(parser: P, separator: Q) -> SkipSepBy<P, Q> {
    SkipSepBy { parser, separator }
}
