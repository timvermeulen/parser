use super::*;

pub struct SepByIter<P, Q, I> {
    parser: P,
    separator: Q,
    start: bool,
    input: I,
}

impl<P, Q, I> Iterator for SepByIter<P, Q, I>
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
            (&mut self.separator)
                .followed_by(&mut self.parser)
                .attempt()
                .map(|(_, o)| o)
                .parse_mut(self.input.borrow_mut())
        }
    }

    // TODO: implement `try_fold`
}

pub fn iter<P, Q, I>(parser: P, separator: Q, input: I) -> SepByIter<P, Q, I>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
    I: std::borrow::BorrowMut<P::Input>,
{
    SepByIter {
        parser,
        separator,
        start: true,
        input,
    }
}
