use super::*;

pub struct ManyIter<P, I> {
    parser: P,
    input: I,
}

impl<P, I> Iterator for ManyIter<P, I>
where
    P: ParserMut<I>,
    I: Stream,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.parse_mut(&mut self.input)
    }
}

pub fn iter<P, I>(parser: P, input: I) -> ManyIter<P, I>
where
    P: ParserMut<I>,
    I: Stream,
{
    ManyIter { parser, input }
}
