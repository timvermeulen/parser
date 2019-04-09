use super::*;

pub struct Iter<P, Q, I> {
    parser: *mut P,
    separator: *mut Q,
    start: bool,
    input: I,
}

impl<P, Q, I> Iterator for Iter<P, Q, &mut I>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            unsafe { (*self.parser).parse_mut(&mut self.input) }
        } else {
            unsafe { (&mut *self.separator).followed_by(&mut *self.parser) }
                .attempt()
                .map(|(_, o)| o)
                .parse_mut(&mut self.input)
        }
    }

    // TODO: implement `try_fold`
}

#[derive(Copy, Clone)]
pub struct SepBy<P, Q, F> {
    parser: P,
    separator: Q,
    f: F,
}

impl<P, Q, F, I, O> ParserOnce<I> for SepBy<P, Q, F>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    F: FnMut(&mut Iter<P, Q, &mut I>) -> Option<O>,
    I: Stream,
{
    type Output = O;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, Q, F, I, O> ParserMut<I> for SepBy<P, Q, F>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    F: FnMut(&mut Iter<P, Q, &mut I>) -> Option<O>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        (self.f)(&mut Iter {
            parser: &mut self.parser as *mut P,
            separator: &mut self.separator as *mut Q,
            start: true,
            input,
        })
    }
}

impl<P, Q, F, I, O> Parser<I> for SepBy<P, Q, F>
where
    P: Parser<I>,
    Q: Parser<I>,
    F: Fn(&mut Iter<P, Q, &mut I>) -> Option<O>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let mut iter = Iter {
            parser: &self.parser as *const P as *mut P,
            separator: &self.separator as *const Q as *mut Q,
            start: true,
            input,
        };
        (self.f)(&mut iter)
    }
}

pub fn sep_by<P, Q, F>(parser: P, separator: Q, f: F) -> SepBy<P, Q, F> {
    SepBy {
        parser,
        separator,
        f,
    }
}
