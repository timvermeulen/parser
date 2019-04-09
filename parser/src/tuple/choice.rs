use super::*;

pub trait ChoiceParserOnce<Input> {
    type Output;

    fn parse_choice_once(self, input: &mut Input) -> Option<Self::Output>;
}

pub trait ChoiceParserMut<Input>: ChoiceParserOnce<Input> {
    fn parse_choice_mut(&mut self, input: &mut Input) -> Option<Self::Output>;
}

pub trait ChoiceParser<Input>: ChoiceParserMut<Input> {
    fn parse_choice(&self, input: &mut Input) -> Option<Self::Output>;
}

pub struct Choice<P, O> {
    parser: P,
    _marker: PhantomData<O>,
}

impl<P, O> Copy for Choice<P, O> where P: Copy {}

impl<P, O> Clone for Choice<P, O>
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

impl<P, I, O> ParserOnce<I> for Choice<P, O>
where
    P: ChoiceParserOnce<I, Output = O>,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_choice_once(input)
    }
}

impl<P, I, O> ParserMut<I> for Choice<P, O>
where
    P: ChoiceParserMut<I, Output = O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_choice_mut(input)
    }
}

impl<P, I, O> Parser<I> for Choice<P, O>
where
    P: ChoiceParser<I, Output = O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser.parse_choice(input)
    }
}

pub fn choice<P, I, O>(parser: P) -> Choice<P, O>
where
    P: ChoiceParserOnce<I, Output = O>,
{
    Choice {
        parser,
        _marker: PhantomData,
    }
}

impl<P1, P2, I, O> ChoiceParserOnce<I> for (P1, P2)
where
    P1: ParserOnce<I, Output = O>,
    P2: ParserOnce<I, Output = O>,
    I: Stream,
{
    type Output = O;

    fn parse_choice_once(self, input: &mut I) -> Option<Self::Output> {
        self.0.or(self.1).parse_once(input)
    }
}

impl<P1, P2, I, O> ChoiceParserMut<I> for (P1, P2)
where
    P1: ParserMut<I, Output = O>,
    P2: ParserMut<I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0.by_mut_ref().or(&mut self.1).parse_mut(input)
    }
}

impl<P1, P2, I, O> ChoiceParser<I> for (P1, P2)
where
    P1: Parser<I, Output = O>,
    P2: Parser<I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut I) -> Option<Self::Output> {
        self.0.by_ref().or(&self.1).parse(input)
    }
}

impl<P1, P2, P3, I, O> ChoiceParserOnce<I> for (P1, P2, P3)
where
    P1: ParserOnce<I, Output = O>,
    P2: ParserOnce<I, Output = O>,
    P3: ParserOnce<I, Output = O>,
    I: Stream,
{
    type Output = O;

    fn parse_choice_once(self, input: &mut I) -> Option<Self::Output> {
        self.0.or(choice((self.1, self.2))).parse_once(input)
    }
}

impl<P1, P2, P3, I, O> ChoiceParserMut<I> for (P1, P2, P3)
where
    P1: ParserMut<I, Output = O>,
    P2: ParserMut<I, Output = O>,
    P3: ParserMut<I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .or(choice((&mut self.1, &mut self.2)))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, I, O> ChoiceParser<I> for (P1, P2, P3)
where
    P1: Parser<I, Output = O>,
    P2: Parser<I, Output = O>,
    P3: Parser<I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut I) -> Option<Self::Output> {
        self.0.by_ref().or(choice((&self.1, &self.2))).parse(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParserOnce<I> for (P1, P2, P3, P4)
where
    P1: ParserOnce<I, Output = O>,
    P2: ParserOnce<I, Output = O>,
    P3: ParserOnce<I, Output = O>,
    P4: ParserOnce<I, Output = O>,
    I: Stream,
{
    type Output = O;

    fn parse_choice_once(self, input: &mut I) -> Option<Self::Output> {
        self.0
            .or(choice((self.1, self.2, self.3)))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParserMut<I> for (P1, P2, P3, P4)
where
    P1: ParserMut<I, Output = O>,
    P2: ParserMut<I, Output = O>,
    P3: ParserMut<I, Output = O>,
    P4: ParserMut<I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .or(choice((&mut self.1, &mut self.2, &mut self.3)))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParser<I> for (P1, P2, P3, P4)
where
    P1: Parser<I, Output = O>,
    P2: Parser<I, Output = O>,
    P3: Parser<I, Output = O>,
    P4: Parser<I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_ref()
            .or(choice((&self.1, &self.2, &self.3)))
            .parse(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParserOnce<I> for (P1, P2, P3, P4, P5)
where
    P1: ParserOnce<I, Output = O>,
    P2: ParserOnce<I, Output = O>,
    P3: ParserOnce<I, Output = O>,
    P4: ParserOnce<I, Output = O>,
    P5: ParserOnce<I, Output = O>,
    I: Stream,
{
    type Output = O;

    fn parse_choice_once(self, input: &mut I) -> Option<Self::Output> {
        self.0
            .or(choice((self.1, self.2, self.3, self.4)))
            .parse_once(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParserMut<I> for (P1, P2, P3, P4, P5)
where
    P1: ParserMut<I, Output = O>,
    P2: ParserMut<I, Output = O>,
    P3: ParserMut<I, Output = O>,
    P4: ParserMut<I, Output = O>,
    P5: ParserMut<I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_mut_ref()
            .or(choice((&mut self.1, &mut self.2, &mut self.3, &mut self.4)))
            .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParser<I> for (P1, P2, P3, P4, P5)
where
    P1: Parser<I, Output = O>,
    P2: Parser<I, Output = O>,
    P3: Parser<I, Output = O>,
    P4: Parser<I, Output = O>,
    P5: Parser<I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .by_ref()
            .or(choice((&self.1, &self.2, &self.3, &self.4)))
            .parse(input)
    }
}
