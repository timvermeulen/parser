use super::*;

pub trait ChainParserOnce<Input> {
    type Output;

    fn parse_chain_once(self, input: &mut Input) -> Option<Self::Output>;
}

pub trait ChainParserMut<Input>: ChainParserOnce<Input> {
    fn parse_chain_mut(&mut self, input: &mut Input) -> Option<Self::Output>;
}

pub trait ChainParser<Input>: ChainParserMut<Input> {
    fn parse_chain(&self, input: &mut Input) -> Option<Self::Output>;
}

pub struct Chain<P, O> {
    parser: P,
    _marker: PhantomData<O>,
}

impl<P, O> Copy for Chain<P, O> where P: Copy {}

impl<P, O> Clone for Chain<P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            _marker: PhantomData,
        }
    }
}

impl<P, I, O> ParserOnce<I> for Chain<P, O>
where
    P: ChainParserOnce<I, Output = O>,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_chain_once(input)
    }
}

impl<P, I, O> ParserMut<I> for Chain<P, O>
where
    P: ChainParserMut<I, Output = O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_chain_mut(input)
    }
}

impl<P, I, O> Parser<I> for Chain<P, O>
where
    P: ChainParser<I, Output = O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_chain(input)
    }
}

pub fn chain<P, I, O>(parser: P) -> Chain<P, O>
where
    P: ChainParserOnce<I, Output = O>,
{
    Chain {
        parser,
        _marker: PhantomData,
    }
}

impl<P1, P2, I> ChainParserOnce<I> for (P1, P2)
where
    P1: ParserOnce<I>,
    P2: ParserOnce<I>,
{
    type Output = (P1::Output, P2::Output);

    fn parse_chain_once(self, input: &mut I) -> Option<Self::Output> {
        self.0.followed_by(self.1).parse_once(input)
    }
}

impl<P1, P2, I> ChainParserMut<I> for (P1, P2)
where
    P1: ParserMut<I>,
    P2: ParserMut<I>,
{
    fn parse_chain_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(&mut self.1)
            .parse_mut(input)
    }
}

impl<P1, P2, I> ChainParser<I> for (P1, P2)
where
    P1: Parser<I>,
    P2: Parser<I>,
{
    fn parse_chain(&self, input: &mut I) -> Option<Self::Output> {
        self.0.by_ref().followed_by(&self.1).parse(input)
    }
}

impl<P1, P2, P3, I> ChainParserOnce<I> for (P1, P2, P3)
where
    P1: ParserOnce<I>,
    P2: ParserOnce<I>,
    P3: ParserOnce<I>,
{
    type Output = (P1::Output, P2::Output, P3::Output);

    fn parse_chain_once(self, input: &mut I) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2)))
            .map_once(|(a, (b, c))| (a, b, c))
            .parse_once(input)
    }
}

impl<P1, P2, P3, I> ChainParserMut<I> for (P1, P2, P3)
where
    P1: ParserMut<I>,
    P2: ParserMut<I>,
    P3: ParserMut<I>,
{
    fn parse_chain_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2)))
            .map_mut(|(a, (b, c))| (a, b, c))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, I> ChainParser<I> for (P1, P2, P3)
where
    P1: Parser<I>,
    P2: Parser<I>,
    P3: Parser<I>,
{
    fn parse_chain(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2)))
            .map(|(a, (b, c))| (a, b, c))
            .parse(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParserOnce<I> for (P1, P2, P3, P4)
where
    P1: ParserOnce<I>,
    P2: ParserOnce<I>,
    P3: ParserOnce<I>,
    P4: ParserOnce<I>,
{
    type Output = (P1::Output, P2::Output, P3::Output, P4::Output);

    fn parse_chain_once(self, input: &mut I) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2, self.3)))
            .map_once(|(a, (b, c, d))| (a, b, c, d))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParserMut<I> for (P1, P2, P3, P4)
where
    P1: ParserMut<I>,
    P2: ParserMut<I>,
    P3: ParserMut<I>,
    P4: ParserMut<I>,
{
    fn parse_chain_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2, &mut self.3)))
            .map_mut(|(a, (b, c, d))| (a, b, c, d))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParser<I> for (P1, P2, P3, P4)
where
    P1: Parser<I>,
    P2: Parser<I>,
    P3: Parser<I>,
    P4: Parser<I>,
{
    fn parse_chain(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2, &self.3)))
            .map(|(a, (b, c, d))| (a, b, c, d))
            .parse(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParserOnce<I> for (P1, P2, P3, P4, P5)
where
    P1: ParserOnce<I>,
    P2: ParserOnce<I>,
    P3: ParserOnce<I>,
    P4: ParserOnce<I>,
    P5: ParserOnce<I>,
{
    #[allow(clippy::type_complexity)]
    type Output = (P1::Output, P2::Output, P3::Output, P4::Output, P5::Output);

    fn parse_chain_once(self, input: &mut I) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2, self.3, self.4)))
            .map_once(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParserMut<I> for (P1, P2, P3, P4, P5)
where
    P1: ParserMut<I>,
    P2: ParserMut<I>,
    P3: ParserMut<I>,
    P4: ParserMut<I>,
    P5: ParserMut<I>,
{
    fn parse_chain_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2, &mut self.3, &mut self.4)))
            .map_mut(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParser<I> for (P1, P2, P3, P4, P5)
where
    P1: Parser<I>,
    P2: Parser<I>,
    P3: Parser<I>,
    P4: Parser<I>,
    P5: Parser<I>,
{
    fn parse_chain(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2, &self.3, &self.4)))
            .map(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse(input)
    }
}
