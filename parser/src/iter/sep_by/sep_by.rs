use super::*;

pub struct Iter<'a, P, Q, I> {
    parser: &'a P,
    separator: &'a Q,
    start: bool,
    input: &'a mut I,
}

impl<P, Q, I> Iterator for Iter<'_, P, Q, I>
where
    P: Parser<I>,
    Q: Parser<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            self.parser.parse_mut(&mut self.input)
        } else {
            self.separator
                .followed_by(self.parser)
                .attempt()
                .map(|(_, o)| o)
                .parse(&mut self.input)
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
    P: Parser<I>,
    Q: Parser<I>,
    F: FnOnce(Iter<'_, P, Q, I>) -> Option<O>,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        (self.f)(Iter {
            parser: &self.parser,
            separator: &self.separator,
            start: true,
            input,
        })
    }
}

impl<P, Q, F, I, O> ParserMut<I> for SepBy<P, Q, F>
where
    P: Parser<I>,
    Q: Parser<I>,
    F: FnMut(Iter<'_, P, Q, I>) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        (self.f)(Iter {
            parser: &self.parser,
            separator: &self.separator,
            start: true,
            input,
        })
    }
}

impl<P, Q, F, I, O> Parser<I> for SepBy<P, Q, F>
where
    P: Parser<I>,
    Q: Parser<I>,
    F: Fn(Iter<'_, P, Q, I>) -> Option<O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let iter = Iter {
            parser: &self.parser,
            separator: &self.separator,
            start: true,
            input,
        };
        (self.f)(iter)
    }
}

pub fn sep_by<P, Q, F>(parser: P, separator: Q, f: F) -> SepBy<P, Q, F> {
    SepBy {
        parser,
        separator,
        f,
    }
}
