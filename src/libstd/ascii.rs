// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ignore-lexer-test FIXME #15679

//! Operations on ASCII strings and characters

#![unstable = "unsure about placement and naming"]
#![allow(deprecated)]

use core::kinds::Sized;
use fmt;
use iter::Iterator;
use mem;
use option::{Option, Some, None};
use slice::{SlicePrelude, AsSlice};
use str::{Str, StrPrelude};
use string::{String, IntoString};
use vec::Vec;

/// Datatype to hold one ascii character. It wraps a `u8`, with the highest bit always zero.
#[deriving(Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Ascii { chr: u8 }

impl Ascii {
    /// Converts an ascii character into a `u8`.
    #[inline]
    #[unstable = "recently renamed"]
    pub fn as_byte(&self) -> u8 {
        self.chr
    }

    /// Deprecated: use `as_byte` isntead.
    #[deprecated = "use as_byte"]
    pub fn to_byte(self) -> u8 {
        self.as_byte()
    }

    /// Converts an ascii character into a `char`.
    #[inline]
    #[unstable = "recently renamed"]
    pub fn as_char(&self) -> char {
        self.chr as char
    }

    /// Deprecated: use `as_char` isntead.
    #[deprecated = "use as_char"]
    pub fn to_char(self) -> char {
        self.as_char()
    }

    /// Convert to lowercase.
    #[inline]
    #[stable]
    pub fn to_lowercase(&self) -> Ascii {
        Ascii{chr: ASCII_LOWER_MAP[self.chr as uint]}
    }

    /// Convert to uppercase.
    #[inline]
    #[stable]
    pub fn to_uppercase(&self) -> Ascii {
        Ascii{chr: ASCII_UPPER_MAP[self.chr as uint]}
    }

    /// Compares two ascii characters of equality, ignoring case.
    #[inline]
    #[deprecated = "normalize with to_lowercase"]
    pub fn eq_ignore_case(self, other: Ascii) -> bool {
        ASCII_LOWER_MAP[self.chr as uint] == ASCII_LOWER_MAP[other.chr as uint]
    }

    // the following methods are like ctype, and the implementation is inspired by musl

    /// Check if the character is a letter (a-z, A-Z)
    #[inline]
    #[stable]
    pub fn is_alphabetic(&self) -> bool {
        (self.chr >= 0x41 && self.chr <= 0x5A) || (self.chr >= 0x61 && self.chr <= 0x7A)
    }

    /// Check if the character is a number (0-9)
    #[inline]
    #[unstable = "may be renamed"]
    pub fn is_digit(&self) -> bool {
        self.chr >= 0x30 && self.chr <= 0x39
    }

    /// Check if the character is a letter or number
    #[inline]
    #[stable]
    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_digit()
    }

    /// Check if the character is a space or horizontal tab
    #[inline]
    #[experimental = "likely to be removed"]
    pub fn is_blank(&self) -> bool {
        self.chr == b' ' || self.chr == b'\t'
    }

    /// Check if the character is a control character
    #[inline]
    #[stable]
    pub fn is_control(&self) -> bool {
        self.chr < 0x20 || self.chr == 0x7F
    }

    /// Checks if the character is printable (except space)
    #[inline]
    #[experimental = "unsure about naming, or whether this is needed"]
    pub fn is_graph(&self) -> bool {
        (self.chr - 0x21) < 0x5E
    }

    /// Checks if the character is printable (including space)
    #[inline]
    #[unstable = "unsure about naming"]
    pub fn is_print(&self) -> bool {
        (self.chr - 0x20) < 0x5F
    }

    /// Checks if the character is alphabetic and lowercase
    #[inline]
    #[stable]
    pub fn is_lowercase(&self) -> bool {
        (self.chr - b'a') < 26
    }

    /// Checks if the character is alphabetic and uppercase
    #[inline]
    #[stable]
    pub fn is_uppercase(&self) -> bool {
        (self.chr - b'A') < 26
    }

    /// Checks if the character is punctuation
    #[inline]
    #[stable]
    pub fn is_punctuation(&self) -> bool {
        self.is_graph() && !self.is_alphanumeric()
    }

    /// Checks if the character is a valid hex digit
    #[inline]
    #[stable]
    pub fn is_hex(&self) -> bool {
        self.is_digit() || ((self.chr | 32u8) - b'a') < 6
    }
}

impl<'a> fmt::Show for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.chr as char).fmt(f)
    }
}

/// Trait for converting into an ascii type.
#[experimental = "may be replaced by generic conversion traits"]
pub trait AsciiCast<T> {
    /// Convert to an ascii type, panic on non-ASCII input.
    #[inline]
    fn to_ascii(&self) -> T {
        assert!(self.is_ascii());
        unsafe {self.to_ascii_nocheck()}
    }

    /// Convert to an ascii type, return None on non-ASCII input.
    #[inline]
    fn to_ascii_opt(&self) -> Option<T> {
        if self.is_ascii() {
            Some(unsafe { self.to_ascii_nocheck() })
        } else {
            None
        }
    }

    /// Convert to an ascii type, not doing any range asserts
    unsafe fn to_ascii_nocheck(&self) -> T;

    /// Check if convertible to ascii
    fn is_ascii(&self) -> bool;
}

#[experimental = "may be replaced by generic conversion traits"]
impl<'a> AsciiCast<&'a[Ascii]> for &'a [u8] {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> &'a[Ascii] {
        mem::transmute(*self)
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        for b in self.iter() {
            if !b.is_ascii() { return false; }
        }
        true
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl<'a> AsciiCast<&'a [Ascii]> for &'a str {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> &'a [Ascii] {
        mem::transmute(*self)
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        self.bytes().all(|b| b.is_ascii())
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiCast<Ascii> for u8 {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self }
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        *self & 128 == 0u8
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiCast<Ascii> for char {
    #[inline]
    unsafe fn to_ascii_nocheck(&self) -> Ascii {
        Ascii{ chr: *self as u8 }
    }

    #[inline]
    fn is_ascii(&self) -> bool {
        *self as u32 - ('\x7F' as u32 & *self as u32) == 0
    }
}

/// Trait for copyless casting to an ascii vector.
#[experimental = "may be replaced by generic conversion traits"]
pub trait OwnedAsciiCast {
    /// Check if convertible to ascii
    fn is_ascii(&self) -> bool;

    /// Take ownership and cast to an ascii vector.
    /// # Panics
    ///
    /// Panic on non-ASCII input.
    #[inline]
    fn into_ascii(self) -> Vec<Ascii> {
        assert!(self.is_ascii());
        unsafe {self.into_ascii_nocheck()}
    }

    /// Take ownership and cast to an ascii vector. Return None on non-ASCII input.
    #[inline]
    fn into_ascii_opt(self) -> Option<Vec<Ascii>> {
        if self.is_ascii() {
            Some(unsafe { self.into_ascii_nocheck() })
        } else {
            None
        }
    }

    /// Take ownership and cast to an ascii vector.
    /// Does not perform validation checks.
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii>;
}

#[experimental = "may be replaced by generic conversion traits"]
impl OwnedAsciiCast for String {
    #[inline]
    fn is_ascii(&self) -> bool {
        self.as_slice().is_ascii()
    }

    #[inline]
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii> {
        self.into_bytes().into_ascii_nocheck()
    }
}

#[experimental = "may be replaced by generic conversion traits"]
impl OwnedAsciiCast for Vec<u8> {
    #[inline]
    fn is_ascii(&self) -> bool {
        self.as_slice().is_ascii()
    }

    #[inline]
    unsafe fn into_ascii_nocheck(self) -> Vec<Ascii> {
        let v = Vec::from_raw_parts(self.as_ptr() as *mut Ascii,
                                    self.len(),
                                    self.capacity());

        // We forget `self` to avoid freeing it at the end of the scope
        // Otherwise, the returned `Vec` would point to freed memory
        mem::forget(self);
        v
    }
}

/// Trait for converting an ascii type to a string. Needed to convert
/// `&[Ascii]` to `&str`.
#[experimental = "may be replaced by generic conversion traits"]
pub trait AsciiStr for Sized? {
    /// Convert to a string.
    fn as_str_ascii<'a>(&'a self) -> &'a str;

    /// Deprecated: use `to_lowercase`
    #[deprecated="renamed `to_lowercase`"]
    fn to_lower(&self) -> Vec<Ascii>;

    /// Convert to vector representing a lower cased ascii string.
    #[deprecated = "use iterators instead"]
    fn to_lowercase(&self) -> Vec<Ascii>;

    /// Deprecated: use `to_uppercase`
    #[deprecated="renamed `to_uppercase`"]
    fn to_upper(&self) -> Vec<Ascii>;

    /// Convert to vector representing a upper cased ascii string.
    #[deprecated = "use iterators instead"]
    fn to_uppercase(&self) -> Vec<Ascii>;

    /// Compares two Ascii strings ignoring case.
    #[deprecated = "use iterators instead"]
    fn eq_ignore_case(&self, other: &[Ascii]) -> bool;
}

#[experimental = "may be replaced by generic conversion traits"]
impl AsciiStr for [Ascii] {
    #[inline]
    fn as_str_ascii<'a>(&'a self) -> &'a str {
        unsafe { mem::transmute(self) }
    }

    #[inline]
    fn to_lower(&self) -> Vec<Ascii> {
      self.to_lowercase()
    }

    #[inline]
    fn to_lowercase(&self) -> Vec<Ascii> {
        self.iter().map(|a| a.to_lowercase()).collect()
    }

    #[inline]
    fn to_upper(&self) -> Vec<Ascii> {
      self.to_uppercase()
    }

    #[inline]
    fn to_uppercase(&self) -> Vec<Ascii> {
        self.iter().map(|a| a.to_uppercase()).collect()
    }

    #[inline]
    fn eq_ignore_case(&self, other: &[Ascii]) -> bool {
        self.iter().zip(other.iter()).all(|(&a, &b)| a.eq_ignore_case(b))
    }
}

impl IntoString for Vec<Ascii> {
    #[inline]
    fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_bytes()) }
    }
}

/// Trait to convert to an owned byte vector by consuming self
#[experimental = "may be replaced by generic conversion traits"]
pub trait IntoBytes {
    /// Converts to an owned byte vector by consuming self
    fn into_bytes(self) -> Vec<u8>;
}

#[experimental = "may be replaced by generic conversion traits"]
impl IntoBytes for Vec<Ascii> {
    fn into_bytes(self) -> Vec<u8> {
        unsafe {
            let v = Vec::from_raw_parts(self.as_ptr() as *mut u8,
                                        self.len(),
                                        self.capacity());

            // We forget `self` to avoid freeing it at the end of the scope
            // Otherwise, the returned `Vec` would point to freed memory
            mem::forget(self);
            v
        }
    }
}


/// Extension methods for ASCII-subset only operations on owned strings
#[experimental = "would prefer to do this in a more general way"]
pub trait OwnedAsciiExt {
    /// Convert the string to ASCII upper case:
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z',
    /// but non-ASCII letters are unchanged.
    fn into_ascii_upper(self) -> Self;

    /// Convert the string to ASCII lower case:
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z',
    /// but non-ASCII letters are unchanged.
    fn into_ascii_lower(self) -> Self;
}

/// Extension methods for ASCII-subset only operations on string slices
#[experimental = "would prefer to do this in a more general way"]
pub trait AsciiExt<T> for Sized? {
    /// Makes a copy of the string in ASCII upper case:
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z',
    /// but non-ASCII letters are unchanged.
    fn to_ascii_upper(&self) -> T;

    /// Makes a copy of the string in ASCII lower case:
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z',
    /// but non-ASCII letters are unchanged.
    fn to_ascii_lower(&self) -> T;

    /// Check that two strings are an ASCII case-insensitive match.
    /// Same as `to_ascii_lower(a) == to_ascii_lower(b)`,
    /// but without allocating and copying temporary strings.
    fn eq_ignore_ascii_case(&self, other: &Self) -> bool;
}

#[experimental = "would prefer to do this in a more general way"]
impl AsciiExt<String> for str {
    #[inline]
    fn to_ascii_upper(&self) -> String {
        // Vec<u8>::to_ascii_upper() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_ascii_upper()) }
    }

    #[inline]
    fn to_ascii_lower(&self) -> String {
        // Vec<u8>::to_ascii_lower() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_ascii_lower()) }
    }

    #[inline]
    fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        self.as_bytes().eq_ignore_ascii_case(other.as_bytes())
    }
}

#[experimental = "would prefer to do this in a more general way"]
impl OwnedAsciiExt for String {
    #[inline]
    fn into_ascii_upper(self) -> String {
        // Vec<u8>::into_ascii_upper() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(self.into_bytes().into_ascii_upper()) }
    }

    #[inline]
    fn into_ascii_lower(self) -> String {
        // Vec<u8>::into_ascii_lower() preserves the UTF-8 invariant.
        unsafe { String::from_utf8_unchecked(self.into_bytes().into_ascii_lower()) }
    }
}

#[experimental = "would prefer to do this in a more general way"]
impl AsciiExt<Vec<u8>> for [u8] {
    #[inline]
    fn to_ascii_upper(&self) -> Vec<u8> {
        self.iter().map(|&byte| ASCII_UPPER_MAP[byte as uint]).collect()
    }

    #[inline]
    fn to_ascii_lower(&self) -> Vec<u8> {
        self.iter().map(|&byte| ASCII_LOWER_MAP[byte as uint]).collect()
    }

    #[inline]
    fn eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        self.len() == other.len() &&
            self.iter().zip(other.iter()).all(
            |(byte_self, byte_other)| {
                ASCII_LOWER_MAP[*byte_self as uint] ==
                    ASCII_LOWER_MAP[*byte_other as uint]
            })
    }
}

#[experimental = "would prefer to do this in a more general way"]
impl OwnedAsciiExt for Vec<u8> {
    #[inline]
    fn into_ascii_upper(mut self) -> Vec<u8> {
        for byte in self.iter_mut() {
            *byte = ASCII_UPPER_MAP[*byte as uint];
        }
        self
    }

    #[inline]
    fn into_ascii_lower(mut self) -> Vec<u8> {
        for byte in self.iter_mut() {
            *byte = ASCII_LOWER_MAP[*byte as uint];
        }
        self
    }
}

/// Returns a 'default' ASCII and C++11-like literal escape of a `u8`
///
/// The default is chosen with a bias toward producing literals that are
/// legal in a variety of languages, including C++11 and similar C-family
/// languages. The exact rules are:
///
/// - Tab, CR and LF are escaped as '\t', '\r' and '\n' respectively.
/// - Single-quote, double-quote and backslash chars are backslash-escaped.
/// - Any other chars in the range [0x20,0x7e] are not escaped.
/// - Any other chars are given hex escapes.
/// - Unicode escapes are never generated by this function.
#[unstable = "needs to be updated to use an iterator"]
pub fn escape_default(c: u8, f: |u8|) {
    match c {
        b'\t' => { f(b'\\'); f(b't'); }
        b'\r' => { f(b'\\'); f(b'r'); }
        b'\n' => { f(b'\\'); f(b'n'); }
        b'\\' => { f(b'\\'); f(b'\\'); }
        b'\'' => { f(b'\\'); f(b'\''); }
        b'"'  => { f(b'\\'); f(b'"'); }
        b'\x20' ... b'\x7e' => { f(c); }
        _ => {
            f(b'\\');
            f(b'x');
            for &offset in [4u, 0u].iter() {
                match ((c as i32) >> offset) & 0xf {
                    i @ 0 ... 9 => f(b'0' + (i as u8)),
                    i => f(b'a' + (i as u8 - 10)),
                }
            }
        }
    }
}

static ASCII_LOWER_MAP: [u8, ..256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@',

          b'a', b'b', b'c', b'd', b'e', b'f', b'g',
    b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
    b'x', b'y', b'z',

                      b'[', b'\\', b']', b'^', b'_',
    b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
    b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
    b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
    b'x', b'y', b'z', b'{', b'|', b'}', b'~', 0x7f,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
    0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
    0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
    0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
    0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

static ASCII_UPPER_MAP: [u8, ..256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'',
    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z', b'[', b'\\', b']', b'^', b'_',
    b'`',

          b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z',

                      b'{', b'|', b'}', b'~', 0x7f,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
    0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf,
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
    0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
    0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
    0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef,
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];


#[cfg(test)]
mod tests {
    use prelude::*;
    use super::*;
    use char::from_u32;
    use str::StrPrelude;

    macro_rules! v2ascii (
        ( [$($e:expr),*]) => (&[$(Ascii{chr:$e}),*]);
        (&[$($e:expr),*]) => (&[$(Ascii{chr:$e}),*]);
    )

    macro_rules! vec2ascii (
        ($($e:expr),*) => ([$(Ascii{chr:$e}),*].to_vec());
    )

    #[test]
    fn test_ascii() {
        assert_eq!(65u8.to_ascii().to_byte(), 65u8);
        assert_eq!(65u8.to_ascii().to_char(), 'A');
        assert_eq!('A'.to_ascii().to_char(), 'A');
        assert_eq!('A'.to_ascii().to_byte(), 65u8);

        assert_eq!('A'.to_ascii().to_lowercase().to_char(), 'a');
        assert_eq!('Z'.to_ascii().to_lowercase().to_char(), 'z');
        assert_eq!('a'.to_ascii().to_uppercase().to_char(), 'A');
        assert_eq!('z'.to_ascii().to_uppercase().to_char(), 'Z');

        assert_eq!('@'.to_ascii().to_lowercase().to_char(), '@');
        assert_eq!('['.to_ascii().to_lowercase().to_char(), '[');
        assert_eq!('`'.to_ascii().to_uppercase().to_char(), '`');
        assert_eq!('{'.to_ascii().to_uppercase().to_char(), '{');

        assert!('0'.to_ascii().is_digit());
        assert!('9'.to_ascii().is_digit());
        assert!(!'/'.to_ascii().is_digit());
        assert!(!':'.to_ascii().is_digit());

        assert!((0x1fu8).to_ascii().is_control());
        assert!(!' '.to_ascii().is_control());
        assert!((0x7fu8).to_ascii().is_control());

        assert!("banana".chars().all(|c| c.is_ascii()));
        assert!(!"ประเทศไทย中华Việt Nam".chars().all(|c| c.is_ascii()));
    }

    #[test]
    fn test_ascii_vec() {
        let test = &[40u8, 32u8, 59u8];
        let b: &[_] = v2ascii!([40, 32, 59]);
        assert_eq!(test.to_ascii(), b);
        assert_eq!("( ;".to_ascii(), b);
        let v = vec![40u8, 32u8, 59u8];
        assert_eq!(v.as_slice().to_ascii(), b);
        assert_eq!("( ;".to_string().as_slice().to_ascii(), b);

        assert_eq!("abCDef&?#".to_ascii().to_lowercase().into_string(), "abcdef&?#".to_string());
        assert_eq!("abCDef&?#".to_ascii().to_uppercase().into_string(), "ABCDEF&?#".to_string());

        assert_eq!("".to_ascii().to_lowercase().into_string(), "".to_string());
        assert_eq!("YMCA".to_ascii().to_lowercase().into_string(), "ymca".to_string());
        let mixed = "abcDEFxyz:.;".to_ascii();
        assert_eq!(mixed.to_uppercase().into_string(), "ABCDEFXYZ:.;".to_string());

        assert!("aBcDeF&?#".to_ascii().eq_ignore_case("AbCdEf&?#".to_ascii()));

        assert!("".is_ascii());
        assert!("a".is_ascii());
        assert!(!"\u2009".is_ascii());

    }

    #[test]
    fn test_ascii_vec_ng() {
        assert_eq!("abCDef&?#".to_ascii().to_lowercase().into_string(), "abcdef&?#".to_string());
        assert_eq!("abCDef&?#".to_ascii().to_uppercase().into_string(), "ABCDEF&?#".to_string());
        assert_eq!("".to_ascii().to_lowercase().into_string(), "".to_string());
        assert_eq!("YMCA".to_ascii().to_lowercase().into_string(), "ymca".to_string());
        let mixed = "abcDEFxyz:.;".to_ascii();
        assert_eq!(mixed.to_uppercase().into_string(), "ABCDEFXYZ:.;".to_string());
    }

    #[test]
    fn test_owned_ascii_vec() {
        assert_eq!(("( ;".to_string()).into_ascii(), vec2ascii![40, 32, 59]);
        assert_eq!((vec![40u8, 32u8, 59u8]).into_ascii(), vec2ascii![40, 32, 59]);
    }

    #[test]
    fn test_ascii_as_str() {
        let v = v2ascii!([40, 32, 59]);
        assert_eq!(v.as_str_ascii(), "( ;");
    }

    #[test]
    fn test_ascii_into_string() {
        assert_eq!(vec2ascii![40, 32, 59].into_string(), "( ;".to_string());
        assert_eq!(vec2ascii!(40, 32, 59).into_string(), "( ;".to_string());
    }

    #[test]
    fn test_ascii_to_bytes() {
        assert_eq!(vec2ascii![40, 32, 59].into_bytes(), vec![40u8, 32u8, 59u8]);
    }

    #[test] #[should_fail]
    fn test_ascii_vec_panic_u8_slice()  { (&[127u8, 128u8, 255u8]).to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_vec_panic_str_slice() { "zoä华".to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_panic_u8_slice() { 255u8.to_ascii(); }

    #[test] #[should_fail]
    fn test_ascii_panic_char_slice() { 'λ'.to_ascii(); }

    #[test]
    fn test_opt() {
        assert_eq!(65u8.to_ascii_opt(), Some(Ascii { chr: 65u8 }));
        assert_eq!(255u8.to_ascii_opt(), None);

        assert_eq!('A'.to_ascii_opt(), Some(Ascii { chr: 65u8 }));
        assert_eq!('λ'.to_ascii_opt(), None);

        assert_eq!("zoä华".to_ascii_opt(), None);

        let test1 = &[127u8, 128u8, 255u8];
        assert_eq!((test1).to_ascii_opt(), None);

        let v = [40u8, 32u8, 59u8];
        let v2: &[_] = v2ascii!(&[40, 32, 59]);
        assert_eq!(v.to_ascii_opt(), Some(v2));
        let v = [127u8, 128u8, 255u8];
        assert_eq!(v.to_ascii_opt(), None);

        let v = "( ;";
        assert_eq!(v.to_ascii_opt(), Some(v2));
        assert_eq!("zoä华".to_ascii_opt(), None);

        assert_eq!((vec![40u8, 32u8, 59u8]).into_ascii_opt(), Some(vec2ascii![40, 32, 59]));
        assert_eq!((vec![127u8, 128u8, 255u8]).into_ascii_opt(), None);

        assert_eq!(("( ;".to_string()).into_ascii_opt(), Some(vec2ascii![40, 32, 59]));
        assert_eq!(("zoä华".to_string()).into_ascii_opt(), None);
    }

    #[test]
    fn test_to_ascii_upper() {
        assert_eq!("url()URL()uRl()ürl".to_ascii_upper(), "URL()URL()URL()üRL".to_string());
        assert_eq!("hıKß".to_ascii_upper(), "HıKß".to_string());

        let mut i = 0;
        while i <= 500 {
            let upper = if 'a' as u32 <= i && i <= 'z' as u32 { i + 'A' as u32 - 'a' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().as_slice().to_ascii_upper(),
                       (from_u32(upper).unwrap()).to_string())
            i += 1;
        }
    }

    #[test]
    fn test_to_ascii_lower() {
        assert_eq!("url()URL()uRl()Ürl".to_ascii_lower(), "url()url()url()Ürl".to_string());
        // Dotted capital I, Kelvin sign, Sharp S.
        assert_eq!("HİKß".to_ascii_lower(), "hİKß".to_string());

        let mut i = 0;
        while i <= 500 {
            let lower = if 'A' as u32 <= i && i <= 'Z' as u32 { i + 'a' as u32 - 'A' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().as_slice().to_ascii_lower(),
                       (from_u32(lower).unwrap()).to_string())
            i += 1;
        }
    }

    #[test]
    fn test_into_ascii_upper() {
        assert_eq!(("url()URL()uRl()ürl".to_string()).into_ascii_upper(),
                   "URL()URL()URL()üRL".to_string());
        assert_eq!(("hıKß".to_string()).into_ascii_upper(), "HıKß".to_string());

        let mut i = 0;
        while i <= 500 {
            let upper = if 'a' as u32 <= i && i <= 'z' as u32 { i + 'A' as u32 - 'a' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().into_ascii_upper(),
                       (from_u32(upper).unwrap()).to_string())
            i += 1;
        }
    }

    #[test]
    fn test_into_ascii_lower() {
        assert_eq!(("url()URL()uRl()Ürl".to_string()).into_ascii_lower(),
                   "url()url()url()Ürl".to_string());
        // Dotted capital I, Kelvin sign, Sharp S.
        assert_eq!(("HİKß".to_string()).into_ascii_lower(), "hİKß".to_string());

        let mut i = 0;
        while i <= 500 {
            let lower = if 'A' as u32 <= i && i <= 'Z' as u32 { i + 'a' as u32 - 'A' as u32 }
                        else { i };
            assert_eq!((from_u32(i).unwrap()).to_string().into_ascii_lower(),
                       (from_u32(lower).unwrap()).to_string())
            i += 1;
        }
    }

    #[test]
    fn test_eq_ignore_ascii_case() {
        assert!("url()URL()uRl()Ürl".eq_ignore_ascii_case("url()url()url()Ürl"));
        assert!(!"Ürl".eq_ignore_ascii_case("ürl"));
        // Dotted capital I, Kelvin sign, Sharp S.
        assert!("HİKß".eq_ignore_ascii_case("hİKß"));
        assert!(!"İ".eq_ignore_ascii_case("i"));
        assert!(!"K".eq_ignore_ascii_case("k"));
        assert!(!"ß".eq_ignore_ascii_case("s"));

        let mut i = 0;
        while i <= 500 {
            let c = i;
            let lower = if 'A' as u32 <= c && c <= 'Z' as u32 { c + 'a' as u32 - 'A' as u32 }
                        else { c };
            assert!((from_u32(i).unwrap()).to_string().as_slice().eq_ignore_ascii_case(
                    (from_u32(lower).unwrap()).to_string().as_slice()));
            i += 1;
        }
    }

    #[test]
    fn test_to_string() {
        let s = Ascii{ chr: b't' }.to_string();
        assert_eq!(s, "t".to_string());
    }

    #[test]
    fn test_show() {
        let c = Ascii { chr: b't' };
        assert_eq!(format!("{}", c), "t".to_string());
    }
}
