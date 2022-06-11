use super::*;

#[derive(Copy, Clone)]
pub struct FromStr<P, O> {
    parser: P,
    _marker: PhantomData<O>,
}

impl<'a, P, I, O> ParserOnce<I> for FromStr<P, O>
where
    P: ParserOnce<I, Output = &'a str>,
    O: std::str::FromStr,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .and_then_once(|s: &str| s.parse().ok())
            .parse_once(input)
    }
}

impl<'a, P, I, O> ParserMut<I> for FromStr<P, O>
where
    P: ParserMut<I, Output = &'a str>,
    O: std::str::FromStr,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_mut_ref()
            .and_then_mut(|s: &str| s.parse().ok())
            .parse_mut(input)
    }
}

impl<'a, P, I, O> Parser<I> for FromStr<P, O>
where
    P: Parser<I, Output = &'a str>,
    O: std::str::FromStr,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parser
            .by_ref()
            .and_then(|s: &str| s.parse().ok())
            .parse(input)
    }
}

pub fn from_str<P, O>(parser: P) -> FromStr<P, O> {
    FromStr {
        parser,
        _marker: PhantomData,
    }
}
