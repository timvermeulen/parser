use super::*;

pub struct IterMut<'a, P, Q, I> {
    parser: &'a mut P,
    separator: &'a mut Q,
    start: bool,
    input: &'a mut I,
}

impl<P, Q, I> Iterator for IterMut<'_, P, Q, I>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            self.parser.parse_mut(&mut self.input)
        } else {
            self.separator
                .followed_by(&mut self.parser)
                .attempt()
                .map_mut(|(_, o)| o)
                .parse_mut(&mut self.input)
        }
    }

    // TODO: implement `try_fold`
}

#[derive(Copy, Clone)]
pub struct SepByMut<P, Q, F> {
    parser: P,
    separator: Q,
    f: F,
}

impl<P, Q, F, I, O> ParserOnce<I> for SepByMut<P, Q, F>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    F: FnOnce(IterMut<'_, P, Q, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        (self.f)(IterMut {
            parser: &mut self.parser,
            separator: &mut self.separator,
            start: true,
            input,
        })
    }
}

impl<P, Q, F, I, O> ParserMut<I> for SepByMut<P, Q, F>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    F: FnMut(IterMut<'_, P, Q, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        (self.f)(IterMut {
            parser: &mut self.parser,
            separator: &mut self.separator,
            start: true,
            input,
        })
    }
}

pub fn sep_by_mut<P, Q, F>(parser: P, separator: Q, f: F) -> SepByMut<P, Q, F> {
    SepByMut {
        parser,
        separator,
        f,
    }
}
