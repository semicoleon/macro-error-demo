#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};

macro_rules! make_enum {
  // creates a variant with a payload
  (case => $name:literal: $case:ident($value:ty); $parser:ident) => {
    if $parser.inner_mut().expect_function_matching($name).is_ok() {
      return Ok(Self::$case($parser.parse()?));
    };
  };

  // creates a variant without a payload
  (case => $name:literal: $case:ident; $parser:ident) => {
    if $parser.inner_mut().expect_ident_matching($name).is_ok() {
      return Ok(Self::$case);
    };
  };

  // parse_ident body piece
  (ident => $name:literal: $case:ident; $ident:ident) => {
    if $ident == $name {
      return Ok(Self::$case);
    }
  };

  // empty parse_ident body for variants with a payload
  (ident => $name:literal: $case:ident($value:ty); $ident:ident) => {};

  // empty parse_fn body for variants without a payload
  (func => $name:literal: $case:ident; $ident:ident, $parser:ident) => {};

  // parse_fn body piece
  (func => $name:literal: $case:ident($value:tt); $ident:ident, $parser:ident) => {
    if $ident == $name {
      return Ok(Self::$case($parser.parse()?));
    }
  };

  // Macro entry point
  (
      $enum_name:ident;
      $($name:literal: $case:ident$(($value:ty))?),+ $(,)?
  ) => {
    #[derive(Debug, PartialEq)]
    enum $enum_name {
      $($case$(($value))?),+
    }

    impl $enum_name
    {
      fn parse_ident(
        ident: &str,
        parser: &str,
      ) -> Result<Self, Error<'static>> {
          let ident = ident.deref();

          $(
            make_enum!(ident => $name: $case $(($value))?; ident);
          )+

          Err(Error::NoMatches)
      }

    fn parse_fn(
        ident: &str,
        parser: &str,
    ) -> Result<Self, Error<'static>> {
        let ident = ident.deref();

      $(
        make_enum!(func => $name: $case $(($value))?; ident, parser);
      )+

      Err(Error::NoMatches)
    }
    }
  }
}

#[derive(PartialEq, Debug)]
enum Error<'a> {
    NoMatches,
    Parse(<Lt<'a> as FromStr>::Err),
}

impl<'a> From<<Lt<'a> as FromStr>::Err> for Error<'a> {
    fn from(from: <Lt as FromStr>::Err) -> Self {
        Self::Parse(from)
    }
}

#[derive(Debug, PartialEq)]
struct Lt<'a>(u8, PhantomData<&'a ()>);

impl<'a> FromStr for Lt<'a> {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?, PhantomData))
    }
}

impl<'a> From<u8> for Lt<'a> {
    fn from(from: u8) -> Self {
        Self(from, PhantomData)
    }
}

#[derive(Debug, PartialEq)]
struct NoLt(u8);

impl FromStr for NoLt {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl From<u8> for NoLt {
    fn from(from: u8) -> Self {
        Self(from)
    }
}

// `expected leaf: `;` rust-analyzer(macro-error)`
make_enum!(
    LtEnum;
    "one": One, "two": Two(Lt<'static>),
);

// No error
make_enum!(
    NoLtEnum;
    "one": One, "two": Two(NoLt)
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lt() {
        assert_eq!(LtEnum::parse_ident("one", ""), Ok(LtEnum::One));
        assert_eq!(LtEnum::parse_fn("two", "2"), Ok(LtEnum::Two(2u8.into())));
    }

    #[test]
    fn no_lt() {
        assert_eq!(NoLtEnum::parse_ident("one", ""), Ok(NoLtEnum::One));
        assert_eq!(
            NoLtEnum::parse_fn("two", "2"),
            Ok(NoLtEnum::Two(2u8.into()))
        );
    }
}
