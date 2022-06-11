use super::*;

#[derive(Copy, Clone)]
pub struct Between<P, L, R> {
    parser: P,
    left: L,
    right: R,
}

impl<P, L, R, I> ParserOnce<I> for Between<P, L, R>
where
    P: ParserOnce<I>,
    L: ParserOnce<I>,
    R: ParserOnce<I>,
{
    type Output = P::Output;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        chain((self.left, self.parser, self.right))
            .map_once(|(_, output, _)| output)
            .parse_once(input)
    }
}

impl<P, L, R, I> ParserMut<I> for Between<P, L, R>
where
    P: ParserMut<I>,
    L: ParserMut<I>,
    R: ParserMut<I>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        chain((&mut self.left, &mut self.parser, &mut self.right))
            .map_mut(|(_, output, _)| output)
            .parse_mut(input)
    }
}

impl<P, L, R, I> Parser<I> for Between<P, L, R>
where
    P: Parser<I>,
    L: Parser<I>,
    R: Parser<I>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        chain((&self.left, &self.parser, &self.right))
            .map(|(_, output, _)| output)
            .parse(input)
    }
}

pub fn between<P, L, R>(parser: P, left: L, right: R) -> Between<P, L, R> {
    Between {
        parser,
        left,
        right,
    }
}
