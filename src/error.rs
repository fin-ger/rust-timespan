// timespan - A simple timespan for chrono times.
//
// Copyright (C) 2017
//     Fin Christensen <christensen.fin@gmail.com>
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

use chrono;
use regex;
use std;
use std::error::Error as StdError;

/// This error describes errors that can occur when operating on spans.
#[derive(Debug)]
pub enum Error {
    /// A span could not be parsed from a string.
    Parsing(chrono::ParseError),
    /// This occurs when a regex failed to compile or match.
    /// This is usually associated with a parsing operation.
    Regex(regex::Error),
    /// The bounds of the span are not in the correct order.
    Ordering,
    /// An operation accessed time slots that are outside the bounds of the span.
    OutOfRange,
    /// The span has a duration of zero.
    Empty,
    /// An operation would split a span in two.
    NotContinuous,
    /// The span has no start time.
    NoStart,
    /// The span has no end time.
    NoEnd,
    /// The local time zone is ambigious.
    LocalAmbigious,
    /// The given string has a bad format.
    BadFormat,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Parsing(ref e) => write!(f, "{}", e),
            Error::Regex(ref e) => write!(f, "{}", e),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parsing(..) => "An error occured while parsing a value",
            Error::Regex(..) => "An error occured while creating a regular expression",
            Error::Ordering => "The left value is not smaller than the right value",
            Error::OutOfRange => "This resulting value is out of range",
            Error::Empty => "The resulting span is empty",
            Error::NotContinuous => "The resulting span is not continuous",
            Error::NoStart => "The resulting span has no start value",
            Error::NoEnd => "The resulting span has no end value",
            Error::LocalAmbigious => "The resulting local time is ambigious",
            Error::BadFormat => "The given string has a bad format",
        }
    }
}

impl std::convert::From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Error::Parsing(e)
    }
}

impl std::convert::From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}
