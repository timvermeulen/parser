mod and_then;
mod attempt;
mod between;
mod flat_map;
mod from_fn;
mod from_str;
mod map;
mod num;
mod optional;
mod recognize;
mod satisfy;
mod satisfy_map;
mod tokens;

use super::*;

pub use from_fn::{from_fn, from_fn_mut, from_fn_once};
pub use num::*;
pub use satisfy::satisfy;
pub use satisfy_map::{satisfy_map, satisfy_map_mut, satisfy_map_once};
pub use tokens::tokens;

pub trait ParserOnce: Sized {
    type Input: Stream;
    type Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output>;

    fn parse_once_and_check_consumed(
        self,
        input: &mut Self::Input,
    ) -> (Option<Self::Output>, bool) {
        let position = input.position();
        let output = self.parse_once(input);
        (output, input.position() != position)
    }

    fn map_once<F, O>(self, f: F) -> map::Map<Self, F>
    where
        F: FnOnce(Self::Output) -> O,
    {
        map::map(self, f)
    }

    fn flat_map_once<P, F>(self, f: F) -> flat_map::FlatMap<Self, F>
    where
        P: ParserOnce<Input = Self::Input>,
        F: FnOnce(Self::Output) -> P,
    {
        flat_map::flat_map(self, f)
    }

    fn and_then_once<F, O>(self, f: F) -> and_then::AndThen<Self, F>
    where
        F: FnOnce(Self::Output) -> Option<O>,
    {
        and_then::and_then(self, f)
    }

    fn optional(self) -> optional::Optional<Self> {
        optional::optional(self)
    }

    fn recognize(self) -> recognize::Recognize<Self> {
        recognize::recognize(self)
    }

    fn from_str<'a, O>(self) -> from_str::FromStr<Self, O>
    where
        Self: ParserOnce<Output = &'a str>,
    {
        from_str::from_str(self)
    }

    fn attempt(self) -> attempt::Attempt<Self> {
        attempt::attempt(self)
    }

    fn between<L, R>(self, left: L, right: R) -> between::Between<Self, L, R>
    where
        L: Parser<Input = Self::Input>,
        R: Parser<Input = Self::Input>,
    {
        between::between(self, left, right)
    }
}

pub trait ParserMut: ParserOnce {
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output>;

    fn parse_mut_and_check_consumed(
        &mut self,
        input: &mut Self::Input,
    ) -> (Option<Self::Output>, bool) {
        let position = input.position();
        let output = self.parse_mut(input);
        (output, input.position() != position)
    }

    fn map_mut<F, O>(self, f: F) -> map::Map<Self, F>
    where
        F: FnMut(Self::Output) -> O,
    {
        map::map(self, f)
    }

    fn flat_map_mut<P, F>(self, f: F) -> flat_map::FlatMap<Self, F>
    where
        P: ParserOnce<Input = Self::Input>,
        F: FnMut(Self::Output) -> P,
    {
        flat_map::flat_map(self, f)
    }

    fn and_then_mut<F, O>(self, f: F) -> and_then::AndThen<Self, F>
    where
        F: FnMut(Self::Output) -> Option<O>,
    {
        and_then::and_then(self, f)
    }

    fn many_mut<F, O>(self, f: F) -> many::ManyMut<Self, F>
    where
        F: FnMut(&mut many::Iter<&mut Self, &mut Self::Input>) -> Option<O>,
    {
        many::many_mut(self, f)
    }

    fn iter_many(self, input: Self::Input) -> many::Iter<Self, Self::Input> {
        many::iter(self, input)
    }

    fn skip_many(self) -> many::SkipMany<Self> {
        many::skip_many(self)
    }

    fn collect_many<I>(self) -> many::CollectMany<Self, I>
    where
        I: FromIterator<Self::Output>,
    {
        many::collect_many(self)
    }

    fn many1_mut<F, O>(self, f: F) -> many1::Many1Mut<Self, F>
    where
        F: FnMut(&mut many1::Iter<&mut Self, &mut Self::Input, Self::Output>) -> Option<O>,
    {
        many1::many1_mut(self, f)
    }

    fn skip_many1(self) -> many1::SkipMany1<Self> {
        many1::skip_many1(self)
    }

    fn collect_many1<I>(self) -> many1::CollectMany1<Self, I>
    where
        I: FromIterator<Self::Output>,
    {
        many1::collect_many1(self)
    }

    fn sep_by_mut<P, F, O>(self, separator: P, f: F) -> sep_by::SepByMut<Self, P, F>
    where
        P: ParserMut<Input = Self::Input>,
        F: FnMut(&mut sep_by::Iter<&mut Self, &mut P, &mut Self::Input>) -> Option<O>,
    {
        sep_by::sep_by_mut(self, separator, f)
    }

    fn iter_sep_by<P>(self, separator: P, input: Self::Input) -> sep_by::Iter<Self, P, Self::Input>
    where
        P: Parser<Input = Self::Input>,
    {
        sep_by::iter(self, separator, input)
    }

    fn skip_sep_by<P>(self, separator: P) -> sep_by::SkipSepBy<Self, P>
    where
        P: Parser<Input = Self::Input>,
    {
        sep_by::skip_sep_by(self, separator)
    }

    fn collect_sep_by<P, I>(self, separator: P) -> sep_by::CollectSepBy<Self, P, I>
    where
        P: Parser<Input = Self::Input>,
        I: FromIterator<Self::Output>,
    {
        sep_by::collect_sep_by(self, separator)
    }

    fn by_mut_ref(&mut self) -> &mut Self {
        self
    }
}

pub trait Parser: ParserMut {
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output>;

    fn parse_and_check_consumed(&self, input: &mut Self::Input) -> (Option<Self::Output>, bool) {
        let position = input.position();
        let output = self.parse(input);
        (output, input.position() != position)
    }

    fn parse_partial(&self, mut input: Self::Input) -> Option<Self::Output> {
        self.parse(&mut input)
    }

    fn parse_to_end(&self, input: Self::Input) -> Option<Self::Output> {
        chain((self, eof())).map(|(o, _)| o).parse_partial(input)
    }

    fn map<F, O>(self, f: F) -> map::Map<Self, F>
    where
        F: Fn(Self::Output) -> O,
    {
        map::map(self, f)
    }

    fn flat_map<P, F>(self, f: F) -> flat_map::FlatMap<Self, F>
    where
        P: ParserOnce<Input = Self::Input>,
        F: Fn(Self::Output) -> P,
    {
        flat_map::flat_map(self, f)
    }

    fn and_then<F, O>(self, f: F) -> and_then::AndThen<Self, F>
    where
        F: Fn(Self::Output) -> Option<O>,
    {
        and_then::and_then(self, f)
    }

    fn many<F, O>(self, f: F) -> many::Many<Self, F>
    where
        F: Fn(&mut many::Iter<&Self, &mut Self::Input>) -> Option<O>,
    {
        many::many(self, f)
    }

    fn many1<F, O>(self, f: F) -> many1::Many1<Self, F>
    where
        F: Fn(&mut many1::Iter<&Self, &mut Self::Input, Self::Output>) -> Option<O>,
    {
        many1::many1(self, f)
    }

    fn sep_by<P, F, O>(self, separator: P, f: F) -> sep_by::SepBy<Self, P, F>
    where
        P: Parser<Input = Self::Input>,
        F: Fn(&mut sep_by::Iter<&Self, &P, &mut Self::Input>) -> Option<O>,
    {
        sep_by::sep_by(self, separator, f)
    }

    fn by_ref(&self) -> &Self {
        self
    }
}

impl<P> ParserOnce for &mut P
where
    P: ParserMut,
{
    type Input = P::Input;
    type Output = P::Output;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P> ParserMut for &mut P
where
    P: ParserMut,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        (*self).parse_mut(input)
    }
}

impl<P> ParserOnce for &P
where
    P: Parser,
{
    type Input = P::Input;
    type Output = P::Output;

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P> ParserMut for &P
where
    P: Parser,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse(input)
    }
}

impl<P> Parser for &P
where
    P: Parser,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        (*self).parse(input)
    }
}

pub fn token<I>(token: I::Item) -> impl Parser<Input = I, Output = I::Item> + Copy
where
    I: Stream,
    I::Item: PartialEq,
{
    satisfy(move |t| t == token)
}

pub fn value<I, O>(output: O) -> impl Parser<Input = I, Output = O> + Copy
where
    I: Stream,
    O: Copy,
{
    from_fn(move |_| Some(output))
}

pub fn any<I>() -> impl Parser<Input = I, Output = I::Item> + Copy
where
    I: Stream,
{
    satisfy_map(Some)
}

pub fn eof<I>() -> impl Parser<Input = I, Output = ()> + Copy
where
    I: Stream,
{
    from_fn(|input: &mut I| if input.is_empty() { Some(()) } else { None })
}

#[derive(Copy, Clone)]
struct String<'a>(&'a str);

impl<'a> ParserOnce for String<'a> {
    type Input = &'a str;
    type Output = ();

    fn parse_once(mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl ParserMut for String<'_> {
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        self.parse(input)
    }
}

impl Parser for String<'_> {
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        tokens(self.0.chars()).parse_once(input)
    }
}

pub fn string(string: &str) -> impl Parser<Input = &str, Output = ()> + Copy {
    String(string)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let mut input = "abcde";
        assert_eq!(string("abc").recognize().parse(&mut input), Some("abc"));
        assert_eq!(input, "de");

        let mut input = "abde";
        assert_eq!(string("abc").recognize().parse(&mut input), None);
        assert_eq!(input, "de");
    }
}
