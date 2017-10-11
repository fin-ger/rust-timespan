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
use chrono::{Duration, NaiveDate};
use chrono::format::{DelayedFormat, StrftimeItems};

impl Spanable for NaiveDate {
    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        NaiveDate::signed_duration_since(self, other)
    }
}

impl Formatable for NaiveDate {
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        NaiveDate::format(self, fmt)
    }
}

impl Parsable for NaiveDate {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<Self, Error> {
        NaiveDate::parse_from_str(s, fmt).map_err(|e| Error::Parsing(e))
    }
}

/// The `NaiveDateSpan` alias is a span consisting of `chrono::NaiveDate`s.
///
/// It can be used to represent date spans that do not depend on a specific time zone
/// (e.g. christmas).
///
/// The `NaiveDateSpan` can be formatted and parsed from a string. It has full
/// support for `serde` serialization and deserialization.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; fn main() {
/// use timespan::NaiveDateSpan;
///
/// let a: NaiveDateSpan = "2017-04-15 - 2017-08-15".parse().unwrap();
/// let b = NaiveDateSpan::parse_from_str(
///     "15.04.17 - 15.08.17",
///     "{start} - {end}",
///     "%d.%m.%y", "%d.%m.%y"
/// ).unwrap();
///
/// let f = a.format("from {start} to {end}", "%m/%d", "%m/%d");
/// assert!(format!("{}", f) == "from 04/15 to 08/15");
/// assert!(a == b);
/// # }
/// ~~~~
pub type NaiveDateSpan = Span<NaiveDate>;
