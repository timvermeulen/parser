use super::*;

pub struct Tokens<Iter, I> {
    iter: Iter,
    _marker: PhantomData<I>,
}

impl<Iter, I> Copy for Tokens<Iter, I> where Iter: Copy {}

impl<Iter, I> Clone for Tokens<Iter, I>
where
    Iter: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Iter, I> ParserOnce<I> for Tokens<Iter, I>
where
    Iter: IntoIterator<Item = I::Item>,
    I: Stream,
    I::Item: PartialEq,
{
    type Output = ();

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.iter
            .into_iter()
            .map(|item| input.uncons_map(|t| if t == item { Some(()) } else { None }))
            .collect()
    }
}

impl<Iter, I> ParserMut<I> for Tokens<Iter, I>
where
    Iter: IntoIterator<Item = I::Item> + Copy,
    I: Stream,
    I::Item: PartialEq,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_once(input)
    }
}

impl<Iter, I> Parser<I> for Tokens<Iter, I>
where
    Iter: IntoIterator<Item = I::Item> + Copy,
    I: Stream,
    I::Item: PartialEq,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        self.parse_once(input)
    }
}

pub fn tokens<Iter, I>(iter: Iter) -> Tokens<Iter, I>
where
    I: Stream,
    I::Item: PartialEq,
    Iter: IntoIterator<Item = I::Item>,
{
    Tokens {
        iter,
        _marker: PhantomData,
    }
}
