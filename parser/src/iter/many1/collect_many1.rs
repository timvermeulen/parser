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

impl<P, O> ParserOnce for CollectMany1<P, O>
where
    P: ParserMut,
    O: FromIterator<P::Output>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, O> ParserMut for CollectMany1<P, O>
where
    P: ParserMut,
    O: FromIterator<P::Output>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .many1_mut(|iter| Some(iter.collect()))
            .parse_mut(input)
    }
}

impl<P, O> Parser for CollectMany1<P, O>
where
    P: Parser,
    O: FromIterator<P::Output>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
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
