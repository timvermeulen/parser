use super::*;

pub struct CollectMany1<P, O> {
    parser: P,
    _marker: PhantomData<O>,
}

impl<P, O> Copy for CollectMany1<P, O> where P: Copy {}

impl<P, O> Clone for CollectMany1<P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        CollectMany1 {
            parser: self.parser.clone(),
            _marker: PhantomData,
        }
    }
}

impl<P, I, O> ParserOnce<I> for CollectMany1<P, O>
where
    P: ParserMut<I>,
    O: FromIterator<P::Output>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I, O> ParserMut<I> for CollectMany1<P, O>
where
    P: ParserMut<I>,
    O: FromIterator<P::Output>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .many1_mut(|iter| Some(iter.collect()))
            .parse_mut(input)
    }
}

impl<P, I, O> Parser<I> for CollectMany1<P, O>
where
    P: Parser<I>,
    O: FromIterator<P::Output>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_ref()
            .many1(|iter| Some(iter.collect()))
            .parse(input)
    }
}

pub fn collect_many1<P, O>(parser: P) -> CollectMany1<P, O> {
    CollectMany1 {
        parser,
        _marker: PhantomData,
    }
}
