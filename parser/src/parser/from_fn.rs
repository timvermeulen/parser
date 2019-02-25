use super::*;

pub struct FromFn<F, I> {
    f: F,
    _marker: PhantomData<I>,
}

impl<F, I> Copy for FromFn<F, I> where F: Copy {}

impl<F, I> Clone for FromFn<F, I>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        FromFn {
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

impl<F, I, O> ParserOnce for FromFn<F, I>
where
    F: FnOnce(&mut I) -> Option<O>,
    I: Stream,
{
    type Input = I;
    type Output = O;

    fn parse_once(self, input: &mut Self::Input) -> Option<Self::Output> {
        (self.f)(input)
    }
}

impl<F, I, O> ParserMut for FromFn<F, I>
where
    F: FnMut(&mut I) -> Option<O>,
    I: Stream,
{
    fn parse_mut(&mut self, input: &mut Self::Input) -> Option<Self::Output> {
        (self.f)(input)
    }
}

impl<F, I, O> Parser for FromFn<F, I>
where
    F: Fn(&mut I) -> Option<O>,
    I: Stream,
{
    fn parse(&self, input: &mut Self::Input) -> Option<Self::Output> {
        (self.f)(input)
    }
}

pub fn from_fn_once<F, I, O>(f: F) -> FromFn<F, I>
where
    F: FnOnce(&mut I) -> Option<O>,
    I: Stream,
{
    FromFn {
        f,
        _marker: PhantomData,
    }
}

pub fn from_fn_mut<F, I, O>(f: F) -> FromFn<F, I>
where
    F: FnMut(&mut I) -> Option<O>,
    I: Stream,
{
    FromFn {
        f,
        _marker: PhantomData,
    }
}

pub fn from_fn<F, I, O>(f: F) -> FromFn<F, I>
where
    F: Fn(&mut I) -> Option<O>,
    I: Stream,
{
    FromFn {
        f,
        _marker: PhantomData,
    }
}
