use super::*;

pub struct SatisfyMap<F, I, O> {
    f: F,
    _marker: PhantomData<(I, O)>,
}

impl<F, I, O> Copy for SatisfyMap<F, I, O> where F: Copy {}

impl<F, I, O> Clone for SatisfyMap<F, I, O>
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

impl<F, I, O> ParserOnce for SatisfyMap<F, I, O>
where
    I: Stream,
    F: FnOnce(I::Item) -> Option<O>,
{
    type Input = I;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        input.uncons_map(self.f)
    }
}

impl<F, I, O> ParserMut for SatisfyMap<F, I, O>
where
    I: Stream,
    F: FnMut(I::Item) -> Option<O>,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        input.uncons_map(&mut self.f)
    }
}

impl<F, I, O> Parser for SatisfyMap<F, I, O>
where
    I: Stream,
    F: Fn(I::Item) -> Option<O>,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        input.uncons_map(&self.f)
    }
}

pub fn satisfy_map_once<F, I, O>(f: F) -> SatisfyMap<F, I, O>
where
    I: Stream,
    F: FnOnce(I::Item) -> Option<O>,
{
    SatisfyMap {
        f,
        _marker: PhantomData,
    }
}

pub fn satisfy_map_mut<F, I, O>(f: F) -> SatisfyMap<F, I, O>
where
    I: Stream,
    F: FnMut(I::Item) -> Option<O>,
{
    SatisfyMap {
        f,
        _marker: PhantomData,
    }
}

pub fn satisfy_map<F, I, O>(f: F) -> SatisfyMap<F, I, O>
where
    I: Stream,
    F: Fn(I::Item) -> Option<O>,
{
    SatisfyMap {
        f,
        _marker: PhantomData,
    }
}
