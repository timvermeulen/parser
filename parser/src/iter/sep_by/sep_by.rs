use super::*;

pub struct Iter<P, Q, I> {
    parser: P,
    separator: Q,
    start: bool,
    input: I,
}

impl<P, Q, I> Iterator for Iter<P, Q, I>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    I: BorrowMut<P::Input>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            self.parser.parse_mut(self.input.borrow_mut())
        } else {
            chain((&mut self.separator, &mut self.parser))
                .attempt()
                .map_mut(|(_, o)| o)
                .parse_mut(self.input.borrow_mut())
        }
    }

    // TODO: implement `try_fold`
}

pub fn iter<P, Q, I>(parser: P, separator: Q, input: I) -> Iter<P, Q, I>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    I: std::borrow::BorrowMut<P::Input>,
{
    Iter {
        parser,
        separator,
        start: true,
        input,
    }
}

#[derive(Copy, Clone)]
pub struct SepByMut<P, Q, F> {
    parser: P,
    separator: Q,
    f: F,
}

impl<P, Q, F, O> ParserOnce for SepByMut<P, Q, F>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    F: FnMut(&mut Iter<&mut P, &mut Q, &mut P::Input>) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, F, O> ParserMut for SepByMut<P, Q, F>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    F: FnMut(&mut Iter<&mut P, &mut Q, &mut P::Input>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        (self.f)(&mut Iter {
            parser: &mut self.parser,
            separator: &mut self.separator,
            start: true,
            input,
        })
    }
}

#[derive(Copy, Clone)]
pub struct SepBy<P, Q, F> {
    parser: P,
    separator: Q,
    f: F,
}

impl<P, Q, F, O> ParserOnce for SepBy<P, Q, F>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    F: FnMut(&mut Iter<&P, &Q, &mut P::Input>) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, F, O> ParserMut for SepBy<P, Q, F>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    F: FnMut(&mut Iter<&P, &Q, &mut P::Input>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        (self.f)(&mut Iter {
            parser: &self.parser,
            separator: &self.separator,
            start: true,
            input,
        })
    }
}

impl<P, Q, F, O> Parser for SepBy<P, Q, F>
where
    P: Parser,
    Q: Parser<Input = P::Input>,
    F: Fn(&mut Iter<&P, &Q, &mut P::Input>) -> Option<O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &self.parser,
            separator: &self.separator,
            start: true,
            input,
        };
        (self.f)(&mut iter)
    }
}

pub fn sep_by_mut<P, Q, F>(parser: P, separator: Q, f: F) -> SepByMut<P, Q, F> {
    SepByMut {
        parser,
        separator,
        f,
    }
}

pub fn sep_by<P, Q, F>(parser: P, separator: Q, f: F) -> SepBy<P, Q, F> {
    SepBy {
        parser,
        separator,
        f,
    }
}
