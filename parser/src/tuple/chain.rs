use super::*;

pub trait ChainParserOnce {
    type Input: Stream;
    type Output;

    fn parse_chain_once(self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub trait ChainParserMut: ChainParserOnce {
    fn parse_chain_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub trait ChainParser: ChainParserMut {
    fn parse_chain(&self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub struct Chain<P, I, O> {
    parser: P,
    _marker: PhantomData<(I, O)>,
}

impl<P, I, O> Copy for Chain<P, I, O> where P: Copy {}

impl<P, I, O> Clone for Chain<P, I, O>
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

impl<P, I, O> ParserOnce for Chain<P, I, O>
where
    P: ChainParserOnce<Input = I, Output = O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_chain_once(input)
    }
}

impl<P, I, O> ParserMut for Chain<P, I, O>
where
    P: ChainParserMut<Input = I, Output = O>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_chain_mut(input)
    }
}

impl<P, O> Parser for Chain<P, P::Input, O>
where
    P: ChainParser<Output = O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_chain(input)
    }
}

pub fn chain<P, O>(parser: P) -> Chain<P, P::Input, O>
where
    P: ChainParserOnce<Output = O>,
{
    Chain {
        parser,
        _marker: PhantomData,
    }
}

impl<P1, P2, I> ChainParserOnce for (P1, P2)
where
    P1: ParserOnce<Input = I>,
    P2: ParserOnce<Input = I>,
    I: Stream,
{
    type Input = I;
    type Output = (P1::Output, P2::Output);

    fn parse_chain_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0.followed_by(self.1).parse_once(input)
    }
}

impl<P1, P2, I> ChainParserMut for (P1, P2)
where
    P1: ParserMut<Input = I>,
    P2: ParserMut<Input = I>,
    I: Stream,
{
    fn parse_chain_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(&mut self.1)
            .parse_mut(input)
    }
}

impl<P1, P2, I> ChainParser for (P1, P2)
where
    P1: Parser<Input = I>,
    P2: Parser<Input = I>,
    I: Stream,
{
    fn parse_chain(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0.by_ref().followed_by(&self.1).parse(input)
    }
}

impl<P1, P2, P3, I> ChainParserOnce for (P1, P2, P3)
where
    P1: ParserOnce<Input = I>,
    P2: ParserOnce<Input = I>,
    P3: ParserOnce<Input = I>,
    I: Stream,
{
    type Input = I;
    type Output = (P1::Output, P2::Output, P3::Output);

    fn parse_chain_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2)))
            .map(|(a, (b, c))| (a, b, c))
            .parse_once(input)
    }
}

impl<P1, P2, P3, I> ChainParserMut for (P1, P2, P3)
where
    P1: ParserMut<Input = I>,
    P2: ParserMut<Input = I>,
    P3: ParserMut<Input = I>,
    I: Stream,
{
    fn parse_chain_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2)))
            .map(|(a, (b, c))| (a, b, c))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, I> ChainParser for (P1, P2, P3)
where
    P1: Parser<Input = I>,
    P2: Parser<Input = I>,
    P3: Parser<Input = I>,
    I: Stream,
{
    fn parse_chain(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2)))
            .map(|(a, (b, c))| (a, b, c))
            .parse(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParserOnce for (P1, P2, P3, P4)
where
    P1: ParserOnce<Input = I>,
    P2: ParserOnce<Input = I>,
    P3: ParserOnce<Input = I>,
    P4: ParserOnce<Input = I>,
    I: Stream,
{
    type Input = I;
    type Output = (P1::Output, P2::Output, P3::Output, P4::Output);

    fn parse_chain_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2, self.3)))
            .map(|(a, (b, c, d))| (a, b, c, d))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParserMut for (P1, P2, P3, P4)
where
    P1: ParserMut<Input = I>,
    P2: ParserMut<Input = I>,
    P3: ParserMut<Input = I>,
    P4: ParserMut<Input = I>,
    I: Stream,
{
    fn parse_chain_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2, &mut self.3)))
            .map(|(a, (b, c, d))| (a, b, c, d))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, I> ChainParser for (P1, P2, P3, P4)
where
    P1: Parser<Input = I>,
    P2: Parser<Input = I>,
    P3: Parser<Input = I>,
    P4: Parser<Input = I>,
    I: Stream,
{
    fn parse_chain(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2, &self.3)))
            .map(|(a, (b, c, d))| (a, b, c, d))
            .parse(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParserOnce for (P1, P2, P3, P4, P5)
where
    P1: ParserOnce<Input = I>,
    P2: ParserOnce<Input = I>,
    P3: ParserOnce<Input = I>,
    P4: ParserOnce<Input = I>,
    P5: ParserOnce<Input = I>,
    I: Stream,
{
    type Input = I;
    #[allow(clippy::type_complexity)]
    type Output = (P1::Output, P2::Output, P3::Output, P4::Output, P5::Output);

    fn parse_chain_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .followed_by(chain((self.1, self.2, self.3, self.4)))
            .map(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParserMut for (P1, P2, P3, P4, P5)
where
    P1: ParserMut<Input = I>,
    P2: ParserMut<Input = I>,
    P3: ParserMut<Input = I>,
    P4: ParserMut<Input = I>,
    P5: ParserMut<Input = I>,
    I: Stream,
{
    fn parse_chain_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .followed_by(chain((&mut self.1, &mut self.2, &mut self.3, &mut self.4)))
            .map(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, P5, I> ChainParser for (P1, P2, P3, P4, P5)
where
    P1: Parser<Input = I>,
    P2: Parser<Input = I>,
    P3: Parser<Input = I>,
    P4: Parser<Input = I>,
    P5: Parser<Input = I>,
    I: Stream,
{
    fn parse_chain(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .by_ref()
            .followed_by(chain((&self.1, &self.2, &self.3, &self.4)))
            .map(|(a, (b, c, d, e))| (a, b, c, d, e))
            .parse(input)
    }
}
