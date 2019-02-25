use super::*;

pub fn digit<I>() -> impl Parser<Input = I, Output = u32> + Copy
where
    I: Stream<Item = char>,
{
    satisfy_map(|c: char| c.to_digit(10))
}

macro_rules! unsigned {
    ($x:ident) => {
        pub fn $x<'a>() -> impl Parser<Input = &'a str, Output = $x> + Copy {
            digit().skip_many1().recognize().from_str()
        }
    };
}

unsigned!(u8);
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(u128);

macro_rules! signed {
    ($x:ident) => {
        pub fn $x<'a>() -> impl Parser<Input = &'a str, Output = $x> + Copy {
            chain((token('-').optional(), digit().skip_many1()))
                .recognize()
                .from_str()
        }
    };
}

signed!(i8);
signed!(i16);
signed!(i32);
signed!(i64);
signed!(i128);
