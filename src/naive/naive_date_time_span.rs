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

use crate::Error;
use crate::Formatable;
use crate::Parsable;
use crate::Span;
use crate::Spanable;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, NaiveDateTime};

impl Spanable for NaiveDateTime {
    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        NaiveDateTime::signed_duration_since(self, other)
    }
}

impl Formatable for NaiveDateTime {
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        NaiveDateTime::format(self, fmt)
    }
}

impl Parsable for NaiveDateTime {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<Self, Error> {
        NaiveDateTime::parse_from_str(s, fmt).map_err(|e| Error::Parsing(e))
    }
}

/// The `NaiveDateTimeSpan` alias is a span consisting of `chrono::NaiveDateTime`s.
///
/// It can be used to represent datetime spans that do not depend on a specific time zone.
///
/// The `NaiveDateTimeSpan` can be formatted and parsed from a string. It has full
/// support for `serde` serialization and deserialization.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; fn main() {
/// use timespan::NaiveDateTimeSpan;
///
/// let a: NaiveDateTimeSpan = "2017-02-20T11:30:00 - 2017-02-23T18:00:00".parse().unwrap();
/// let b = NaiveDateTimeSpan::parse_from_str(
///     "02/20/17 11.30 am - 02/23/17 06.00 pm",
///     "{start} - {end}",
///     "%D %I.%M %p", "%D %I.%M %p"
/// ).unwrap();
///
/// let f = a.format("from {start} to {end}", "%R on %A", "%R on %A");
/// assert!(format!("{}", f) == "from 11:30 on Monday to 18:00 on Thursday");
/// assert!(a == b);
/// # }
/// ~~~~
pub type NaiveDateTimeSpan = Span<NaiveDateTime>;
