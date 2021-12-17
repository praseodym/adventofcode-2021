// boilerplate to let nom work with ux
// https://fasterthanli.me/series/making-our-own-ping/part-9
// https://fasterthanli.me/series/making-our-own-ping/part-11

use std::ops::RangeFrom;

use nom::bits::complete::take as take_bits;
use nom::combinator::map;
use nom::error::{ErrorKind as NomErrorKind, ParseError as NomParseError};
use nom::{ErrorConvert, Slice};
use ux::*;

pub type BitInput<'a> = (&'a [u8], usize);
pub type BitResult<'a, T> = nom::IResult<BitInput<'a>, T, Error<BitInput<'a>>>;

#[derive(Debug)]
pub struct Error<I> {
    pub errors: Vec<(I, NomErrorKind)>,
}

impl<I> NomParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: NomErrorKind) -> Self {
        let errors = vec![(input, kind)];
        Self { errors }
    }

    fn append(input: I, kind: NomErrorKind, mut other: Self) -> Self {
        other.errors.push((input, kind));
        other
    }
}

pub fn remaining(input: BitInput) -> usize {
    input.0.len() * 8 - input.1
}

pub trait BitParsable
where
    Self: Sized,
{
    fn parse(i: BitInput) -> BitResult<Self>;
}

// impl BitParsable for u4 {
//     fn parse(i: BitInput) -> BitResult<Self> {
//         map(take_bits(4_usize), Self::new)(i)
//     }
// }

macro_rules! impl_bit_parsable_for_ux {
    ($width: expr) => {
        paste::item! {
            impl BitParsable for [<u $width>] {
                fn parse(i: BitInput) -> BitResult<Self> {
                    map(take_bits($width as usize), Self::new)(i)
                }
            }
        }
    };
}

impl<I> ErrorConvert<Error<I>> for Error<(I, usize)>
where
    I: Slice<RangeFrom<usize>>,
{
    fn convert(self) -> Error<I> {
        let errors = self
            .errors
            .into_iter()
            .map(|((rest, offset), err)| (rest.slice(offset / 8..), err))
            .collect();
        Error { errors }
    }
}

impl_bit_parsable_for_ux!(1);
impl_bit_parsable_for_ux!(3);
impl_bit_parsable_for_ux!(4);
impl_bit_parsable_for_ux!(5);
impl_bit_parsable_for_ux!(11);
impl_bit_parsable_for_ux!(15);
