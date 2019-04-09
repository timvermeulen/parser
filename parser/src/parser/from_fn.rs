use super::*;

pub struct FromFn<F> {
    f: F,
}

impl<F> Copy for FromFn<F> where F: Copy {}

impl<F> Clone for FromFn<F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        FromFn { f: self.f.clone() }
    }
}

impl<F, I, O> ParserOnce<I> for FromFn<F>
where
    F: FnOnce(&mut I) -> Option<O>,
{
    type Output = O;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        (self.f)(input)
    }
}

impl<F, I, O> ParserMut<I> for FromFn<F>
where
    F: FnMut(&mut I) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        (self.f)(input)
    }
}

impl<F, I, O> Parser<I> for FromFn<F>
where
    F: Fn(&mut I) -> Option<O>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        (self.f)(input)
    }
}

pub fn from_fn_once<F, I, O>(f: F) -> FromFn<F>
where
    F: FnOnce(&mut I) -> Option<O>,
{
    FromFn { f }
}

pub fn from_fn_mut<F, I, O>(f: F) -> FromFn<F>
where
    F: FnMut(&mut I) -> Option<O>,
{
    FromFn { f }
}

pub fn from_fn<F, I, O>(f: F) -> FromFn<F>
where
    F: Fn(&mut I) -> Option<O>,
{
    FromFn { f }
}
