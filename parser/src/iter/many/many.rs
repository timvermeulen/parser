use super::*;

pub struct Iter<P, I> {
    parser: *mut P,
    input: I,
}

impl<P, I> Iterator for Iter<P, &mut I>
where
    P: ParserMut<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { (*self.parser).parse_mut(self.input) }
    }
}

#[derive(Copy, Clone)]
pub struct Many<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for Many<P, F>
where
    P: ParserMut<I>,
    F: FnMut(&mut Iter<P, &mut I>) -> Option<O>,
    I: Stream,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, I, O> ParserMut<I> for Many<P, F>
where
    P: ParserMut<I>,
    F: FnMut(&mut Iter<P, &mut I>) -> Option<O>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &mut self.parser as *mut P,
            input,
        };
        (self.f)(&mut iter)
    }
}

impl<P, F, I, O> Parser<I> for Many<P, F>
where
    P: Parser<I>,
    F: Fn(&mut Iter<P, &mut I>) -> Option<O>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
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
