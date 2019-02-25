use super::*;

#[derive(Copy, Clone)]
pub struct Recognize<P>(P);

impl<P, I> ParserOnce for Recognize<P>
where
    P: ParserOnce<Input = I>,
    I: Stream,
{
    type Input = I;
    type Output = I::Range;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse_once(input)?;
        let end = input.position();
        Some(unsafe { Self::Input::between(start, end) })
    }
}

impl<P, I> ParserMut for Recognize<P>
where
    P: ParserMut<Input = I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse_mut(input)?;
        let end = input.position();
        Some(unsafe { Self::Input::between(start, end) })
    }
}

impl<P, I> Parser for Recognize<P>
where
    P: Parser<Input = I>,
    I: Stream,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        let start = input.position();
        self.0.parse(input)?;
        let end = input.position();
        Some(unsafe { Self::Input::between(start, end) })
    }
}

pub fn recognize<P>(parser: P) -> Recognize<P> {
    Recognize(parser)
}
