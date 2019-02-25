use super::*;

#[derive(Copy, Clone)]
pub struct SkipSepBy<P, Q> {
    parser: P,
    separator: Q,
}

impl<P, Q> ParserOnce for SkipSepBy<P, Q>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
{
    type Input = P::Input;
    type Output = ();

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q> ParserMut for SkipSepBy<P, Q>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .sep_by_mut(&mut self.separator, |iter| {
                iter.for_each(drop);
                Some(())
            })
            .parse_mut(input)
    }
}

impl<P, Q> Parser for SkipSepBy<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
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
