// timespan - A simple timespan for chrono times.
//
// Copyright (C) 2017
//     Fin Christensen <fin.christensen@posteo.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::Error;
use chrono::format::{DelayedFormat, ParseError, StrftimeItems};
use chrono::Duration;
use std::clone::Clone;
use std::cmp::{Ord, PartialOrd};
use std::fmt::Display;
use std::marker::Copy;
use std::ops::{Add, Sub};
use std::str::FromStr;

/// Types that implement this trait can be used inside a `Span`.
pub trait Spanable:
    Copy + Clone + Ord + PartialOrd + Add<Duration, Output = Self> + Sub<Duration, Output = Self>
{
    /// This is a wrapper method to the `signed_duration_since` method from `chrono`.
    fn signed_duration_since(self, _: Self) -> Duration;
}

/// Spanable types that are parsable can be used to deserialize a given string
/// to a span instance.
pub trait Parsable: FromStr<Err = ParseError> {
    /// This is a wrapper method to the `parse_from_str` method from `chrono`.
    fn parse_from_str(_: &str, _: &str) -> Result<Self, Error>
    where
        Self: Sized;
}

/// Spanable types that are formatable can be used to serialize a given span
/// to a string.
pub trait Formatable: Display {
    /// This is a wrapper method to the `format` method from `chrono`.
    fn format<'a>(&self, _: &'a str) -> DelayedFormat<StrftimeItems<'a>>;
}
