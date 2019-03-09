use super::*;

pub struct Iter<P, I> {
    parser: *mut P,
    input: I,
}

impl<P, I> Iterator for Iter<P, I>
where
    P: ParserMut,
    I: BorrowMut<P::Input>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { (*self.parser).parse_mut(self.input.borrow_mut()) }
    }
}

#[derive(Copy, Clone)]
pub struct Many<P, F> {
    parser: P,
    f: F,
}

impl<P, F, O> ParserOnce for Many<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<P, &mut P::Input>) -> Option<O>,
{
    type Input = P::Input;
    type Output = O;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, O> ParserMut for Many<P, F>
where
    P: ParserMut,
    F: FnMut(&mut Iter<P, &mut P::Input>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &mut self.parser as *mut P,
            input,
        };
        (self.f)(&mut iter)
    }
}

impl<P, F, O> Parser for Many<P, F>
where
    P: Parser,
    F: Fn(&mut Iter<P, &mut P::Input>) -> Option<O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &self.parser as *const P as *mut P,
            input,
        };
        (self.f)(&mut iter)
    }
}

pub fn many<P, F>(parser: P, f: F) -> Many<P, F> {
    Many { parser, f }
}
