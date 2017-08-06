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

#[derive(Debug)]
pub enum Error {
    Parsing(chrono::ParseError),
    Regex(regex::Error),
    Ordering,
    OutOfRange,
    Empty,
    NotContinuous,
    NoStart,
    NoEnd,
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
