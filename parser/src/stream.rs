mod is_empty {
    pub fn str_is_empty(string: &str) -> bool {
        string.is_empty()
    }

    pub fn slice_is_empty<T>(slice: &[T]) -> bool {
        slice.is_empty()
    }
}

pub trait Stream: Copy {
    type Item: Copy;
    type Position: Ord;
    type Range;

    fn is_empty(&self) -> bool;
    fn position(&self) -> Self::Position;
    fn uncons_map<O>(&mut self, f: impl FnOnce(Self::Item) -> Option<O>) -> Option<O>;
    unsafe fn between(start: Self::Position, end: Self::Position) -> Self::Range;
}

impl<'a> Stream for &'a str {
    type Item = char;
    type Position = *const u8;
    type Range = Self;

    fn is_empty(&self) -> bool {
        is_empty::str_is_empty(self)
    }

    fn position(&self) -> Self::Position {
        self.as_ptr()
    }

    fn uncons_map<O>(&mut self, f: impl FnOnce(Self::Item) -> Option<O>) -> Option<O> {
        let mut chars = self.chars();
        let output = chars.next().and_then(f)?;
        *self = chars.as_str();
        Some(output)
    }

    unsafe fn between(start: Self::Position, end: Self::Position) -> Self::Range {
        std::str::from_utf8_unchecked(<(&[u8])>::between(start, end))
    }
}

impl<'a, T> Stream for &'a [T] {
    type Item = &'a T;
    type Position = *const T;
    type Range = Self;

    fn is_empty(&self) -> bool {
        is_empty::slice_is_empty(self)
    }

    fn position(&self) -> Self::Position {
        self.as_ptr()
    }

    fn uncons_map<O>(&mut self, f: impl FnOnce(Self::Item) -> Option<O>) -> Option<O> {
        let (first, rest) = self.split_first()?;
        let output = f(first)?;
        *self = rest;
        Some(output)
    }

    unsafe fn between(start: Self::Position, end: Self::Position) -> Self::Range {
        std::slice::from_raw_parts(start, end as usize - start as usize)
    }
}
