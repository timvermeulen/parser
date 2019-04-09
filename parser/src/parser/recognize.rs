use super::*;

#[derive(Copy, Clone)]
pub struct Recognize<P>(P);

impl<P, I> ParserOnce<I> for Recognize<P>
where
    P: ParserOnce<I>,
    I: Stream,
{
    type Output = I::Range;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse_once(input)?;
        let end = input.position();
        Some(unsafe { I::between(start, end) })
    }
}

impl<P, I> ParserMut<I> for Recognize<P>
where
    P: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse_mut(input)?;
        let end = input.position();
        Some(unsafe { I::between(start, end) })
    }
}

impl<P, I> Parser<I> for Recognize<P>
where
    P: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse(input)?;
        let end = input.position();
        Some(unsafe { I::between(start, end) })
    }
}

pub fn recognize<P>(parser: P) -> Recognize<P> {
    Recognize(parser)
}
