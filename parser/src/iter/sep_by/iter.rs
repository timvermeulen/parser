use super::*;

pub struct SepByIter<P, Q, I> {
    parser: P,
    separator: Q,
    start: bool,
    input: I,
}

impl<P, Q, I> Iterator for SepByIter<P, Q, I>
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
            (&mut self.separator)
                .followed_by(&mut self.parser)
                .attempt()
                .map(|(_, o)| o)
                .parse_mut(&mut self.input)
        }
    }

    // TODO: implement `try_fold`
}

pub fn iter<P, Q, I>(parser: P, separator: Q, input: I) -> SepByIter<P, Q, I>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
{
    SepByIter {
        parser,
        separator,
        start: true,
        input,
    }
}
