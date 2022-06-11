use super::*;

pub struct Iter<'a, P, I> {
    parser: &'a P,
    input: &'a mut I,
}

impl<P, I> Iterator for Iter<'_, P, I>
where
    P: Parser<I>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse(self.input)
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
    F: FnMut(&mut Iter<'_, P, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, F, I, O> ParserMut<I> for Many<P, F>
where
    P: ParserMut<I>,
    F: FnMut(&mut Iter<'_, P, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &self.parser,
            input,
        };
        (self.f)(&mut iter)
    }
}

impl<P, F, I, O> Parser<I> for Many<P, F>
where
    P: Parser<I>,
    F: Fn(&mut Iter<'_, P, I>) -> Option<O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &self.parser,
            input,
        };
        (self.f)(&mut iter)
    }
}

pub fn many<P, F>(parser: P, f: F) -> Many<P, F> {
    Many { parser, f }
}
