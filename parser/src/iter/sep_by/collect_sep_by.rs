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

impl<P, Q, I, O> ParserOnce<I> for CollectSepBy<P, Q, O>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, I, O> ParserMut<I> for CollectSepBy<P, Q, O>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .sep_by_mut(&mut self.separator, |iter| Some(iter.collect()))
            .parse_mut(input)
    }
}

impl<P, Q, I, O> Parser<I> for CollectSepBy<P, Q, O>
where
    P: Parser<I>,
    Q: Parser<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
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
