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
use crate::NaiveDateSpan;
use crate::Span;
use crate::Spanable;
use chrono::{Date, Duration, TimeZone};
use chrono::format::{DelayedFormat, StrftimeItems};
use std;

impl<T: TimeZone> Spanable for Date<T>
where
    <T as TimeZone>::Offset: std::marker::Copy,
{
    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        Date::signed_duration_since(self, other)
    }
}

impl<T: TimeZone> Formatable for Date<T>
where
    <T as TimeZone>::Offset: std::fmt::Display,
{
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        Date::format(self, fmt)
    }
}

/// The `DateSpan` alias is a span consisting of `chrono::Date`s.
///
/// It can be used to represent date spans that depend on a specific time zone.
///
/// The `DateSpan` can be formatted to a string. It can only be used for serialization
/// when using `serde` as deserialization is not supported.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; extern crate chrono_tz; fn main() {
/// use timespan::DateSpan;
/// use chrono_tz::Europe::Paris;
///
/// let a = DateSpan::from_utc_datespan(
///     &"1789-06-17 - 1799-11-10".parse().unwrap(),
///     &Paris,
/// );
///
/// let f = a.format(
///     "The french revolution lasted from the {start} to the {end}.",
///     "%eth of %B %Y",
///     "%eth of %B %Y",
/// );
/// assert!(
///     format!("{}", f) ==
///     "The french revolution lasted from the 17th of June 1789 to the 10th of November 1799."
/// );
/// # }
/// ~~~~
pub type DateSpan<T> = Span<Date<T>>;

impl<T: TimeZone> DateSpan<T> {
    /// Create a `DateSpan` from a `NaiveDateSpan` with the time zone set to the local time zone.
    ///
    /// Currently the result handling of the internally used `TimeZone::from_local_date` is not
    /// implemented properly. Therefore only date spans with a single local time zone can be created.
    /// Ambigious local time zones will lead to `Error::LocalAmbigious`.
    ///
    /// To avoid this `from_utc_datespan` should be prefered.
    pub fn from_local_datespan(span: &NaiveDateSpan, tz: &T) -> Result<Self, Error> {
        Ok(DateSpan {
            start: tz.from_local_date(&span.start).single().ok_or(
                Error::LocalAmbigious,
            )?,
            end: tz.from_local_date(&span.end).single().ok_or(
                Error::LocalAmbigious,
            )?,
        })
    }

    /// Create a `DateSpan` from a `NaiveDateSpan` with the time zone set to UTC.
    ///
    /// As a `DateSpan` cannot be parsed from a string this method is the preferred way of creating
    /// a `DateSpan`.
    ///
    /// # Example
    ///
    /// ~~~~
    /// # extern crate timespan; extern crate chrono_tz; fn main() {
    /// use timespan::DateSpan;
    /// use chrono_tz::Europe::Berlin;
    ///
    /// let a = DateSpan::from_utc_datespan(
    ///     &"2017-05-21 - 2017-05-27".parse().unwrap(),
    ///     &Berlin,
    /// );
    ///
    /// assert!(format!("{}", a) == "2017-05-21CEST - 2017-05-27CEST");
    /// # }
    /// ~~~~
    pub fn from_utc_datespan(span: &NaiveDateSpan, tz: &T) -> Self {
        DateSpan {
            start: tz.from_utc_date(&span.start),
            end: tz.from_utc_date(&span.end),
        }
    }
}
