#![feature(try_trait, specialization)]
#![warn(clippy::all)]
#![allow(clippy::module_inception)]

mod iter;
mod parser;
mod stream;
mod tuple;

pub use self::parser::*;
use iter::{many, many1, sep_by};
pub use parser_macros::opaque;
pub use stream::Stream;
pub use tuple::{chain, choice};

use std::borrow::BorrowMut;
use std::iter::FromIterator;
use std::marker::{PhantomData, Sized};

pub mod prelude {
    pub use crate::{
        opaque,
        parser::{
            from_fn, from_fn_mut, from_fn_once, satisfy, satisfy_map, string, token, tokens,
            Parser, ParserMut, ParserOnce,
        },
        tuple::{chain, choice},
    };

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    fn assert_parse<P>(
        parser: P,
        mut input: P::Input,
        output: Option<P::Output>,
        remainder: P::Input,
    ) where
        P: Parser,
        P::Input: Debug + PartialEq,
        P::Output: Debug + PartialEq,
    {
        assert_eq!(parser.parse(&mut input), output);
        assert_eq!(input, remainder);
    }

    #[test]
    fn test_satisfy_map() {
        assert_parse(digit(), "", None, "");
        assert_parse(digit(), "a1", None, "a1");
        assert_parse(digit(), "1a", Some(1), "a");
    }

    #[test]
    fn test_or() {
        assert_parse(choice((string("aa"), string("ab"))), "abc", None, "bc");
    }

    #[test]
    fn test_attempt() {
        assert_parse(
            choice((string("aa").attempt(), string("ab"))).recognize(),
            "abc",
            Some("ab"),
            "c",
        );
    }

    #[test]
    fn test_chain() {
        assert_parse(
            chain((super::u32(), string("ab").recognize())),
            "123abc",
            Some((123, "ab")),
            "c",
        );
    }

    #[test]
    fn test_number() {
        assert_parse(super::u32(), "123abc", Some(123), "abc");
        assert_parse(super::u32(), "123123123123abc", None, "abc");
        assert_parse(super::u32(), "abc", None, "abc");
        assert_parse(super::i32(), "123abc", Some(123), "abc");
        assert_parse(super::i32(), "123123123123abc", None, "abc");
        assert_parse(super::i32(), "-123abc", Some(-123), "abc");
        assert_parse(super::i32(), "-123123123123abc", None, "abc");
        assert_parse(super::i32(), "abc", None, "abc");
    }

    #[test]
    fn test_many() {
        let mut input = "abcabcabcde";
        let vec: Vec<_> = string("abc")
            .recognize()
            .collect_many()
            .parse(&mut input)
            .unwrap();
        assert_eq!(vec, vec!["abc", "abc", "abc"]);
        assert_eq!(input, "de");
    }

    #[test]
    fn test_sep_by() {
        let mut input = "abc123abc26abde";
        let vec = string("abc")
            .recognize()
            .collect_sep_by(super::u32())
            .parse(&mut input);
        assert_eq!(vec, Some(vec!["abc", "abc"]));
        assert_eq!(input, "26abde");
    }

    #[test]
    fn test_iter_many() {
        let input = "abcabcabcde";

        for string in string("abc").recognize().iter_many(input) {
            dbg!(string);
        }
    }

    #[test]
    fn test_recursive() {
        #[allow(unused)]
        enum JSON {
            Array(Vec<JSON>),
            Object(Vec<(String, JSON)>),
            Number(i32),
            String(String),
        }

        #[opaque]
        fn json<'a>() -> impl Parser<Input = &'a str, Output = JSON> {
            choice((
                parser::i32().map(JSON::Number),
                string().map(JSON::String),
                array().map(JSON::Array),
                object().map(JSON::Object),
            ))
        }

        #[allow(unused)]
        fn string<'a>() -> impl Parser<Input = &'a str, Output = String> {
            satisfy(char::is_alphabetic)
                .skip_many1()
                .recognize()
                .from_str()
                .between(token('\"'), token('\"'))
        }

        #[allow(unused)]
        fn array<'a>() -> impl Parser<Input = &'a str, Output = Vec<JSON>> {
            json()
                .collect_sep_by(token(','))
                .between(token('['), token(']'))
        }

        #[allow(unused)]
        fn object<'a>() -> impl Parser<Input = &'a str, Output = Vec<(String, JSON)>> {
            chain((string(), token(':'), json()))
                .map(|(key, _, value)| (key, value))
                .collect_sep_by(token(','))
                .between(token('{'), token('}'))
        }
    }
}
