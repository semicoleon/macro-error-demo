A minimal(ish) reproduction of a rust-analyzer diagnostic. 
The error printed by `rust-analyzer diagnostics` is

```rust
Diagnostic { code: DiagnosticCode("macro-error"), message: "expected leaf: `;`", range: 2593..2660, severity: Error, unused: false, experimental: true, fixes: None }
```

The error seems to depend on whether the enum variant payload has generic parameters.

`cargo check` and `cargo test` both complete without issues.

## `cargo expand` output

```rust
#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};
enum Error<'a> {
    NoMatches,
    Parse(<Lt<'a> as FromStr>::Err),
}
impl<'a> ::core::marker::StructuralPartialEq for Error<'a> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::cmp::PartialEq for Error<'a> {
    #[inline]
    fn eq(&self, other: &Error<'a>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Error::Parse(ref __self_0), &Error::Parse(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Error<'a>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Error::Parse(ref __self_0), &Error::Parse(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::fmt::Debug for Error<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Error::NoMatches,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "NoMatches");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Error::Parse(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Parse");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl<'a> From<<Lt<'a> as FromStr>::Err> for Error<'a> {
    fn from(from: <Lt as FromStr>::Err) -> Self {
        Self::Parse(from)
    }
}
struct Lt<'a>(u8, PhantomData<&'a ()>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::fmt::Debug for Lt<'a> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Lt(ref __self_0_0, ref __self_0_1) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Lt");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl<'a> ::core::marker::StructuralPartialEq for Lt<'a> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<'a> ::core::cmp::PartialEq for Lt<'a> {
    #[inline]
    fn eq(&self, other: &Lt<'a>) -> bool {
        match *other {
            Lt(ref __self_1_0, ref __self_1_1) => match *self {
                Lt(ref __self_0_0, ref __self_0_1) => {
                    (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Lt<'a>) -> bool {
        match *other {
            Lt(ref __self_1_0, ref __self_1_1) => match *self {
                Lt(ref __self_0_0, ref __self_0_1) => {
                    (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1)
                }
            },
        }
    }
}
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
struct NoLt(u8);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for NoLt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            NoLt(ref __self_0_0) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "NoLt");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for NoLt {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for NoLt {
    #[inline]
    fn eq(&self, other: &NoLt) -> bool {
        match *other {
            NoLt(ref __self_1_0) => match *self {
                NoLt(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &NoLt) -> bool {
        match *other {
            NoLt(ref __self_1_0) => match *self {
                NoLt(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
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
enum LtEnum {
    One,
    Two(Lt<'static>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for LtEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&LtEnum::One,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "One");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&LtEnum::Two(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Two");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for LtEnum {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for LtEnum {
    #[inline]
    fn eq(&self, other: &LtEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&LtEnum::Two(ref __self_0), &LtEnum::Two(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &LtEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&LtEnum::Two(ref __self_0), &LtEnum::Two(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
impl LtEnum {
    fn parse_ident(ident: &str, parser: &str) -> Result<Self, Error<'static>> {
        let ident = ident.deref();
        if ident == "one" {
            return Ok(Self::One);
        };
        Err(Error::NoMatches)
    }
    fn parse_fn(ident: &str, parser: &str) -> Result<Self, Error<'static>> {
        let ident = ident.deref();
        if ident == "two" {
            return Ok(Self::Two(parser.parse()?));
        };
        Err(Error::NoMatches)
    }
}
enum NoLtEnum {
    One,
    Two(NoLt),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for NoLtEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&NoLtEnum::One,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "One");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&NoLtEnum::Two(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Two");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for NoLtEnum {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for NoLtEnum {
    #[inline]
    fn eq(&self, other: &NoLtEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&NoLtEnum::Two(ref __self_0), &NoLtEnum::Two(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &NoLtEnum) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&NoLtEnum::Two(ref __self_0), &NoLtEnum::Two(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
impl NoLtEnum {
    fn parse_ident(ident: &str, parser: &str) -> Result<Self, Error<'static>> {
        let ident = ident.deref();
        if ident == "one" {
            return Ok(Self::One);
        };
        Err(Error::NoMatches)
    }
    fn parse_fn(ident: &str, parser: &str) -> Result<Self, Error<'static>> {
        let ident = ident.deref();
        if ident == "two" {
            return Ok(Self::Two(parser.parse()?));
        };
        Err(Error::NoMatches)
    }
}
```