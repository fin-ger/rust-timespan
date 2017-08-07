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

use chrono::{ParseError, ParseResult, Duration};
use chrono::format::{DelayedFormat, StrftimeItems};
use std::cmp::{PartialOrd, Ord};
use std::clone::Clone;
use std::ops::{Add, Sub};
use std::fmt::Display;
use std::str::FromStr;

pub trait Spanable: Display + Copy + Clone +
    FromStr<Err = ParseError> +
    Ord + PartialOrd +
    Add<Duration, Output = Self> + Sub<Duration, Output = Self>
{
    fn format<'a>(&self, &'a str) -> DelayedFormat<StrftimeItems<'a>>;

    fn signed_duration_since(self, Self) -> Duration;

    fn parse_from_str(&str, &str) -> ParseResult<Self>;
}
