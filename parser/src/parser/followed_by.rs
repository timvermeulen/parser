use super::*;

pub struct FollowedBy<P, Q>(P, Q);

impl<P, Q> ParserOnce for FollowedBy<P, Q>
where
    P: ParserOnce,
    Q: ParserOnce<Input = P::Input>,
{
    type Input = P::Input;
    type Output = (P::Output, Q::Output);

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        let q = self.1;
        self.0
            .parse_once(input)
            .and_then(|output1| q.parse_once(input).map(move |output2| (output1, output2)))
    }
}

impl<P, Q> ParserMut for FollowedBy<P, Q>
where
    P: ParserMut,
    Q: ParserMut<Input = P::Input>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0.parse_mut(input).and_then(|output1| {
            self.1
                .parse_mut(input)
                .map(move |output2| (output1, output2))
        })
    }
}

impl<P, Q> Parser for FollowedBy<P, Q>
where
    P: Parser,
    Q: Parser<Input = P::Input>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        self.0
            .parse(input)
            .and_then(|output1| self.1.parse(input).map(move |output2| (output1, output2)))
    }
}

pub fn followed_by<P, Q>(p: P, q: Q) -> FollowedBy<P, Q> {
    FollowedBy(p, q)
}
