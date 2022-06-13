use super::*;

pub struct IterMut<'a, P, I> {
    parser: &'a mut P,
    input: &'a mut I,
}

impl<P, I> Iterator for IterMut<'_, P, I>
where
    P: ParserMut<I>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_mut(self.input)
    }
}

#[derive(Copy, Clone)]
pub struct ManyMut<P, F> {
    parser: P,
    f: F,
}

impl<P, F, I, O> ParserOnce<I> for ManyMut<P, F>
where
    P: ParserMut<I>,
    F: FnOnce(IterMut<'_, P, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        let iter = IterMut {
            parser: &mut self.parser,
            input,
        };
        (self.f)(iter)
    }
}

impl<P, F, I, O> ParserMut<I> for ManyMut<P, F>
where
    P: ParserMut<I>,
    F: FnMut(IterMut<'_, P, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let iter = IterMut {
            parser: &mut self.parser,
            input,
        };
        (self.f)(iter)
    }
}

pub fn many_mut<P, F>(parser: P, f: F) -> ManyMut<P, F> {
    ManyMut { parser, f }
}
