use super::*;

#[derive(Copy, Clone)]
pub struct FollowedBy<P, Q>(P, Q);

impl<P, Q, I> ParserOnce<I> for FollowedBy<P, Q>
where
    P: ParserOnce<I>,
    Q: ParserOnce<I>,
    I: Stream,
{
    type Output = (P::Output, Q::Output);

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        let q = self.1;
        self.0
            .parse_once(input)
            .and_then(|output1| q.parse_once(input).map(move |output2| (output1, output2)))
    }
}

impl<P, Q, I> ParserMut<I> for FollowedBy<P, Q>
where
    P: ParserMut<I>,
    Q: ParserMut<I>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.0.parse_mut(input).and_then(|output1| {
            self.1
                .parse_mut(input)
                .map(move |output2| (output1, output2))
        })
    }
}

impl<P, Q, I> Parser<I> for FollowedBy<P, Q>
where
    P: Parser<I>,
    Q: Parser<I>,
    I: Stream,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.0
            .parse(input)
            .and_then(|output1| self.1.parse(input).map(move |output2| (output1, output2)))
    }
}

pub fn followed_by<P, Q>(p: P, q: Q) -> FollowedBy<P, Q> {
    FollowedBy(p, q)
}
