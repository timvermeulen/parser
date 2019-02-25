use super::*;

pub struct CollectSepBy<P, Q, O> {
    parser: P,
    separator: Q,
    _marker: PhantomData<O>,
}

impl<P, Q, O> Copy for CollectSepBy<P, Q, O>
where
    P: Copy,
    Q: Copy,
{
}

impl<P, Q, O> Clone for CollectSepBy<P, Q, O>
where
    P: Clone,
    Q: Clone,
{
    fn clone(&self) -> Self {
        CollectSepBy {
            parser: self.parser.clone(),
            separator: self.separator.clone(),
            _marker: PhantomData,
        }
    }
}

impl<P, Q, O> ParserOnce for CollectSepBy<P, Q, O>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    O: FromIterator<P::Output>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, O> ParserMut for CollectSepBy<P, Q, O>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    O: FromIterator<P::Output>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .sep_by_mut(&mut self.separator, |iter| Some(iter.collect()))
            .parse_mut(input)
    }
}

impl<P, Q, O> Parser for CollectSepBy<P, Q, O>
where
    P: Parser,
    Q: Parser<Input = P::Input>,
    O: FromIterator<P::Output>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .by_ref()
            .sep_by(&self.separator, |iter| Some(iter.collect()))
            .parse(input)
    }
}

pub fn collect_sep_by<P, Q, O>(parser: P, separator: Q) -> CollectSepBy<P, Q, O> {
    CollectSepBy {
        parser,
        separator,
        _marker: PhantomData,
    }
}
