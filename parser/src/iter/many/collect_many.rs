use super::*;

pub struct CollectMany<P, O> {
    parser: P,
    _marker: PhantomData<O>,
}

impl<P, O> Copy for CollectMany<P, O> where P: Copy {}

impl<P, O> Clone for CollectMany<P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        CollectMany {
            parser: self.parser.clone(),
            _marker: PhantomData,
        }
    }
}

impl<P, I, O> ParserOnce<I> for CollectMany<P, O>
where
    P: ParserMut<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I, O> ParserMut<I> for CollectMany<P, O>
where
    P: ParserMut<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .many_mut(|iter| Some(iter.collect()))
            .parse_mut(input)
    }
}

impl<P, I, O> Parser<I> for CollectMany<P, O>
where
    P: Parser<I>,
    I: Stream,
    O: FromIterator<P::Output>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_ref()
            .many(|iter| Some(iter.collect()))
            .parse(input)
    }
}

pub fn collect_many<P, O>(parser: P) -> CollectMany<P, O> {
    CollectMany {
        parser,
        _marker: PhantomData,
    }
}
