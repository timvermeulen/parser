use super::*;

pub trait ChoiceParserOnce {
    type Input: Stream;
    type Output;

    fn parse_choice_once(self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub trait ChoiceParserMut: ChoiceParserOnce {
    fn parse_choice_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub trait ChoiceParser: ChoiceParserMut {
    fn parse_choice(&self, input: &mut Self::Input) -> Option<Self::Output>;
}

pub struct Choice<P, I, O> {
    parser: P,
    _marker: PhantomData<(I, O)>,
}

impl<P, I, O> Copy for Choice<P, I, O> where P: Copy {}

impl<P, I, O> Clone for Choice<P, I, O>
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

impl<P, O> ParserOnce for Choice<P, P::Input, O>
where
    P: ChoiceParserOnce<Output = O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_choice_once(input)
    }
}

impl<P, O> ParserMut for Choice<P, P::Input, O>
where
    P: ChoiceParserMut<Output = O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_choice_mut(input)
    }
}

impl<P, O> Parser for Choice<P, P::Input, O>
where
    P: ChoiceParser<Output = O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parser.parse_choice(input)
    }
}

pub fn choice<P, O>(parser: P) -> Choice<P, P::Input, O>
where
    P: ChoiceParserOnce<Output = O>,
{
    Choice {
        parser,
        _marker: PhantomData,
    }
}

impl<P1, P2, I, O> ChoiceParserOnce for (P1, P2)
where
    P1: ParserOnce<Input = I, Output = O>,
    P2: ParserOnce<Input = I, Output = O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_choice_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_once_and_check_consumed(input) {
            (None, false) => self.1.parse_once(input),
            (result, _) => result,
        }
    }
}

impl<P1, P2, I, O> ChoiceParserMut for (P1, P2)
where
    P1: ParserMut<Input = I, Output = O>,
    P2: ParserMut<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_mut_and_check_consumed(input) {
            (None, false) => self.1.parse_mut(input),
            (result, _) => result,
        }
    }
}

impl<P1, P2, I, O> ChoiceParser for (P1, P2)
where
    P1: Parser<Input = I, Output = O>,
    P2: Parser<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut Self::Input) -> Option<Self::Output> {
        match self.0.parse_and_check_consumed(input) {
            (None, false) => self.1.parse(input),
            (result, _) => result,
        }
    }
}

impl<P1, P2, P3, I, O> ChoiceParserOnce for (P1, P2, P3)
where
    P1: ParserOnce<Input = I, Output = O>,
    P2: ParserOnce<Input = I, Output = O>,
    P3: ParserOnce<Input = I, Output = O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_choice_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((self.0, choice((self.1, self.2)))).parse_once(input)
    }
}

impl<P1, P2, P3, I, O> ChoiceParserMut for (P1, P2, P3)
where
    P1: ParserMut<Input = I, Output = O>,
    P2: ParserMut<Input = I, Output = O>,
    P3: ParserMut<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((&mut self.0, choice((&mut self.1, &mut self.2)))).parse_mut(input)
    }
}

impl<P1, P2, P3, I, O> ChoiceParser for (P1, P2, P3)
where
    P1: Parser<Input = I, Output = O>,
    P2: Parser<Input = I, Output = O>,
    P3: Parser<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((&self.0, choice((&self.1, &self.2)))).parse(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParserOnce for (P1, P2, P3, P4)
where
    P1: ParserOnce<Input = I, Output = O>,
    P2: ParserOnce<Input = I, Output = O>,
    P3: ParserOnce<Input = I, Output = O>,
    P4: ParserOnce<Input = I, Output = O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_choice_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((self.0, choice((self.1, self.2, self.3)))).parse_once(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParserMut for (P1, P2, P3, P4)
where
    P1: ParserMut<Input = I, Output = O>,
    P2: ParserMut<Input = I, Output = O>,
    P3: ParserMut<Input = I, Output = O>,
    P4: ParserMut<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((&mut self.0, choice((&mut self.1, &mut self.2, &mut self.3)))).parse_mut(input)
    }
}

impl<P1, P2, P3, P4, I, O> ChoiceParser for (P1, P2, P3, P4)
where
    P1: Parser<Input = I, Output = O>,
    P2: Parser<Input = I, Output = O>,
    P3: Parser<Input = I, Output = O>,
    P4: Parser<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((&self.0, choice((&self.1, &self.2, &self.3)))).parse(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParserOnce for (P1, P2, P3, P4, P5)
where
    P1: ParserOnce<Input = I, Output = O>,
    P2: ParserOnce<Input = I, Output = O>,
    P3: ParserOnce<Input = I, Output = O>,
    P4: ParserOnce<Input = I, Output = O>,
    P5: ParserOnce<Input = I, Output = O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_choice_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((self.0, choice((self.1, self.2, self.3, self.4)))).parse_once(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParserMut for (P1, P2, P3, P4, P5)
where
    P1: ParserMut<Input = I, Output = O>,
    P2: ParserMut<Input = I, Output = O>,
    P3: ParserMut<Input = I, Output = O>,
    P4: ParserMut<Input = I, Output = O>,
    P5: ParserMut<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((
            &mut self.0,
            choice((&mut self.1, &mut self.2, &mut self.3, &mut self.4)),
        ))
        .parse_mut(input)
    }
}

impl<P1, P2, P3, P4, P5, I, O> ChoiceParser for (P1, P2, P3, P4, P5)
where
    P1: Parser<Input = I, Output = O>,
    P2: Parser<Input = I, Output = O>,
    P3: Parser<Input = I, Output = O>,
    P4: Parser<Input = I, Output = O>,
    P5: Parser<Input = I, Output = O>,
    I: Stream,
{
    fn parse_choice(&self, input: &mut Self::Input) -> Option<Self::Output> {
        choice((&self.0, choice((&self.1, &self.2, &self.3, &self.4)))).parse(input)
    }
}
