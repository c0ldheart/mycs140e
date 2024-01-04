// Copyright 2012-2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode string slices.
//!
//! The `&str` type is one of the two main string types, the other being `String`.
//! Unlike its `String` counterpart, its contents are borrowed.
//!
//! # Basic Usage
//!
//! A basic string declaration of `&str` type:
//!
//! ```
//! let hello_world = "Hello, World!";
//! ```
//!
//! Here we have declared a string literal, also known as a string slice.
//! String literals have a static lifetime, which means the string `hello_world`
//! is guaranteed to be valid for the duration of the entire program.
//! We can explicitly specify `hello_world`'s lifetime as well:
//!
//! ```
//! let hello_world: &'static str = "Hello, world!";
//! ```
//!
//! *[See also the `str` primitive type](../../std/primitive.str.html).*

#![stable(feature = "rust1", since = "1.0.0")]

// Many of the usings in this module are only used in the test configuration.
// It's cleaner to just turn off the unused_imports warning than to fix them.
#![allow(unused_imports)]

use core::fmt;
use core::str as core_str;
use core::str::pattern::Pattern;
use core::str::pattern::{Searcher, ReverseSearcher, DoubleEndedSearcher};
use core::mem;
use core::iter::FusedIterator;

use core::alloc;

// use vec_deque::VecDeque;
// use borrow::{Borrow, ToOwned};

// use std_unicode;
// use alloc::vec;
use slice::{SliceConcatExt, SliceIndex};
// use boxed::Box;


#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{FromStr, Utf8Error};
#[allow(deprecated)]
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{Lines, LinesAny};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{Split, RSplit};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{SplitN, RSplitN};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{SplitTerminator, RSplitTerminator};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{Matches, RMatches};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{MatchIndices, RMatchIndices};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{from_utf8, from_utf8_mut, Chars, CharIndices, Bytes};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::{from_utf8_unchecked, from_utf8_unchecked_mut, ParseBoolError};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::SplitWhitespace;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::str::pattern;
#[stable(feature = "encode_utf16", since = "1.8.0")]
pub use core::str::EncodeUtf16;


// #[unstable(feature = "slice_concat_ext",
//            reason = "trait should not have to exist",
//            issue = "27747")]
// impl<S: Borrow<str>> SliceConcatExt<str> for [S] {
//     type Output = String;

//     fn concat(&self) -> String {
//         if self.is_empty() {
//             return String::new();
//         }

//         // `len` calculation may overflow but push_str will check boundaries
//         let len = self.iter().map(|s| s.borrow().len()).sum();
//         let mut result = String::with_capacity(len);

//         for s in self {
//             result.push_str(s.borrow())
//         }

//         result
//     }

//     fn join(&self, sep: &str) -> String {
//         if self.is_empty() {
//             return String::new();
//         }

//         // concat is faster
//         if sep.is_empty() {
//             return self.concat();
//         }

//         // this is wrong without the guarantee that `self` is non-empty
//         // `len` calculation may overflow but push_str but will check boundaries
//         let len = sep.len() * (self.len() - 1) +
//                   self.iter().map(|s| s.borrow().len()).sum::<usize>();
//         let mut result = String::with_capacity(len);
//         let mut first = true;

//         for s in self {
//             if first {
//                 first = false;
//             } else {
//                 result.push_str(sep);
//             }
//             result.push_str(s.borrow());
//         }
//         result
//     }

//     fn connect(&self, sep: &str) -> String {
//         self.join(sep)
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl Borrow<str> for String {
//     #[inline]
//     fn borrow(&self) -> &str {
//         &self[..]
//     }
// }

// #[stable(feature = "rust1", since = "1.0.0")]
// impl ToOwned for str {
//     type Owned = String;
//     fn to_owned(&self) -> String {
//         unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
//     }

//     fn clone_into(&self, target: &mut String) {
//         let mut b = mem::replace(target, String::new()).into_bytes();
//         self.as_bytes().clone_into(&mut b);
//         *target = unsafe { String::from_utf8_unchecked(b) }
//     }
// }

