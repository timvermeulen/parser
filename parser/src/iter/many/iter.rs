use super::*;

pub struct ManyIter<P, I> {
    parser: P,
    input: I,
}

impl<P, I> Iterator for ManyIter<P, I>
where
    P: ParserMut,
    I: BorrowMut<P::Input>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_mut(self.input.borrow_mut())
    }
}

pub fn iter<P, I>(parser: P, input: I) -> ManyIter<P, I>
where
    P: ParserMut,
    I: std::borrow::BorrowMut<P::Input>,
{
    ManyIter { parser, input }
}
