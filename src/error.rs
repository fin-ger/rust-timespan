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
use std::error::Error;

#[derive(Debug)]
pub enum TimespanError {
    Parsing(chrono::ParseError),
    Regex(regex::Error),
    Ordering,
    OutOfRange,
    Empty,
    NotContinuous,
    NoStart,
    NoEnd,
}

impl std::fmt::Display for TimespanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TimespanError::Parsing(ref e) => write!(f, "{}", e),
            TimespanError::Regex(ref e) => write!(f, "{}", e),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl Error for TimespanError {
    fn description(&self) -> &str {
        match *self {
            TimespanError::Parsing(..) => "An error occured while parsing a value",
            TimespanError::Regex(..) => "An error occured while creating a regular expression",
            TimespanError::Ordering => "The left value is not smaller than the right value",
            TimespanError::OutOfRange => "This resulting value is out of range",
            TimespanError::Empty => "The resulting span is empty",
            TimespanError::NotContinuous => "The resulting span is not continuous",
            TimespanError::NoStart => "The resulting span has no start value",
            TimespanError::NoEnd => "The resulting span has no end value",
        }
    }
}

impl std::convert::From<chrono::ParseError> for TimespanError {
    fn from(e: chrono::ParseError) -> Self {
        TimespanError::Parsing(e)
    }
}

impl std::convert::From<regex::Error> for TimespanError {
    fn from(e: regex::Error) -> Self {
        TimespanError::Regex(e)
    }
}
