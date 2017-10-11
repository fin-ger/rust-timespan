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

use Error;
use Formatable;
use Parsable;
use Span;
use Spanable;
use chrono::{Duration, NaiveTime};
use chrono::format::{DelayedFormat, StrftimeItems};

impl Spanable for NaiveTime {
    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        NaiveTime::signed_duration_since(self, other)
    }
}

impl Formatable for NaiveTime {
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        NaiveTime::format(self, fmt)
    }
}

impl Parsable for NaiveTime {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<Self, Error> {
        NaiveTime::parse_from_str(s, fmt).map_err(|e| Error::Parsing(e))
    }
}

/// The `NaiveTimeSpan` alias is a span consisting of `chrono::NaiveTime`s.
///
/// It can be used to represent time spans that do not depend on a specific time zone.
///
/// The `NaiveTimeSpan` can be formatted and parsed from a string. It has full
/// support for `serde` serialization and deserialization.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; fn main() {
/// use timespan::NaiveTimeSpan;
///
/// let a: NaiveTimeSpan = "17:30:00 - 19:15:00".parse().unwrap();
/// let b = NaiveTimeSpan::parse_from_str(
///     "05.30 PM - 07.15 PM",
///     "{start} - {end}",
///     "%I.%M %P", "%I.%M %P"
/// ).unwrap();
///
/// let f = a.format("from {start} to {end}", "%R", "%R");
/// assert!(format!("{}", f) == "from 17:30 to 19:15");
/// assert!(a == b);
/// # }
/// ~~~~
pub type NaiveTimeSpan = Span<NaiveTime>;
