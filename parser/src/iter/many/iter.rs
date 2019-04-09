use super::*;

pub struct ManyIter<P, I> {
    parser: P,
    input: I,
}

impl<P, I> Iterator for ManyIter<P, I>
where
    P: ParserMut<I>,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_mut(&mut self.input)
    }
}

pub fn iter<P, I>(parser: P, input: I) -> ManyIter<P, I>
where
    P: ParserMut<I>,
{
    ManyIter { parser, input }
}
