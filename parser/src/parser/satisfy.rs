use super::*;

pub struct Satisfy<F, I> {
    f: F,
    _marker: PhantomData<I>,
}

impl<F, I> Copy for Satisfy<F, I> where F: Copy {}

impl<F, I> Clone for Satisfy<F, I>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

impl<F, I> ParserOnce<I> for Satisfy<F, I>
where
    I: Stream,
    F: FnOnce(I::Item) -> bool,
{
    type Output = I::Item;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        satisfy_map_once(move |x| if (self.f)(x) { Some(x) } else { None }).parse_once(input)
    }
}

impl<F, I> ParserMut<I> for Satisfy<F, I>
where
    I: Stream,
    F: FnMut(I::Item) -> bool,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        satisfy_map_mut(move |x| if (self.f)(x) { Some(x) } else { None }).parse_mut(input)
    }
}

impl<F, I> Parser<I> for Satisfy<F, I>
where
    I: Stream,
    F: Fn(I::Item) -> bool,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        satisfy_map(move |x| if (self.f)(x) { Some(x) } else { None }).parse(input)
    }
}

pub fn satisfy<F, I>(f: F) -> Satisfy<F, I>
where
    F: Fn(I::Item) -> bool,
    I: Stream,
{
    Satisfy {
        f,
        _marker: PhantomData,
    }
}
