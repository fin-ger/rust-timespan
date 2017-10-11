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
use NaiveDateTimeSpan;
use Parsable;
use Span;
use Spanable;
use chrono::{DateTime as ChronoDateTime, Duration, TimeZone};
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::offset::{FixedOffset, Local, Utc};
use std;

impl<T: TimeZone> Spanable for ChronoDateTime<T>
where
    <T as TimeZone>::Offset: std::marker::Copy,
{
    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        ChronoDateTime::signed_duration_since(self, other)
    }
}

impl<T: TimeZone> Formatable for ChronoDateTime<T>
where
    <T as TimeZone>::Offset: std::fmt::Display,
{
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        ChronoDateTime::format(self, fmt)
    }
}

impl Parsable for ChronoDateTime<Local> {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<ChronoDateTime<Local>, Error> {
        Local.datetime_from_str(s, fmt).map_err(
            |e| Error::Parsing(e),
        )
    }
}

impl Parsable for ChronoDateTime<Utc> {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<ChronoDateTime<Utc>, Error> {
        Utc.datetime_from_str(s, fmt).map_err(|e| Error::Parsing(e))
    }
}

impl Parsable for ChronoDateTime<FixedOffset> {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> Result<ChronoDateTime<FixedOffset>, Error> {
        ChronoDateTime::parse_from_str(s, fmt).map_err(|e| Error::Parsing(e))
    }
}

/// The `DateTimeSpan` alias is a span consisting of `chrono::DateTime`s.
///
/// It can be used to represent datetime spans that depend on a specific time zone.
///
/// The `DateTimeSpan` can be formatted and parsed from a string. It can be used for serialization
/// and deserialization with `serde`. The deserialization is currently only supported for the
/// `Utc`, `Local` and `FixedOffset` time zones. The time zones provided by `chrono-tz` do not
/// implement `from_str` and `parse_from_str` for `chrono::DateTime<Tz>` and can therefore not be
/// deserialized.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; extern crate chrono; fn main() {
/// use timespan::DateTimeSpan;
/// use chrono::Utc;
///
/// let a: DateTimeSpan<Utc> = "2017-01-01T15:10:00 +0200 - 2017-01-02T09:30:00 +0200"
///    .parse().unwrap();
///
/// assert!(
///     format!("{}", a.format("{start} to {end}", "%c", "%c")) ==
///     "Sun Jan  1 13:10:00 2017 to Mon Jan  2 07:30:00 2017"
/// );
/// # }
/// ~~~~
pub type DateTimeSpan<T> = Span<ChronoDateTime<T>>;

impl<T: TimeZone> DateTimeSpan<T> {
    /// Create a `DateTimeSpan` from a `NaiveDateTimeSpan` with the time zone set to the local time zone.
    ///
    /// Currently the result handling of the internally used `TimeZone::from_local_datetime` is not
    /// implemented properly. Therefore only date spans with a single local time zone can be created.
    /// Ambigious local time zones will lead to `Error::LocalAmbigious`.
    ///
    /// To avoid this `from_utc_datetimespan` should be prefered.
    pub fn from_local_datetimespan(span: &NaiveDateTimeSpan, tz: &T) -> Result<Self, Error> {
        Ok(DateTimeSpan {
            start: tz.from_local_datetime(&span.start).single().ok_or(
                Error::LocalAmbigious,
            )?,
            end: tz.from_local_datetime(&span.end).single().ok_or(
                Error::LocalAmbigious,
            )?,
        })
    }

    /// Create a `DateTimeSpan` from a `NaiveDateTimeSpan` with the time zone set to UTC.
    ///
    /// # Example
    ///
    /// ~~~~
    /// # extern crate timespan; extern crate chrono_tz; fn main() {
    /// use timespan::DateTimeSpan;
    /// use chrono_tz::America::Puerto_Rico;
    ///
    /// let a = DateTimeSpan::from_utc_datetimespan(
    ///     &"2017-03-12T12:00:00 - 2017-03-15T14:00:00".parse().unwrap(),
    ///     &Puerto_Rico,
    /// );
    ///
    /// assert!(format!("{}", a) == "2017-03-12 08:00:00 AST - 2017-03-15 10:00:00 AST");
    /// # }
    /// ~~~~
    pub fn from_utc_datetimespan(span: &NaiveDateTimeSpan, tz: &T) -> Self {
        DateTimeSpan {
            start: tz.from_utc_datetime(&span.start),
            end: tz.from_utc_datetime(&span.end),
        }
    }
}

#[cfg(feature = "with-chrono-tz")]
pub use self::with_chrono_tz::DateTime;

#[cfg(feature = "with-chrono-tz")]
mod with_chrono_tz {
    use super::DateTimeSpan;
    use super::Error;
    use super::Parsable;
    use chrono::{DateTime as ChronoDateTime, ParseError, TimeZone};
    use chrono_tz::Tz;
    use regex::Regex;
    use std::convert::From;
    use std::str::FromStr;

    pub struct DateTime<T: TimeZone>(pub ChronoDateTime<T>);

    impl<T: TimeZone> From<ChronoDateTime<T>> for DateTime<T> {
        fn from(dt: ChronoDateTime<T>) -> DateTime<T> {
            DateTime(dt)
        }
    }

    impl FromStr for DateTime<Tz> {
        type Err = ParseError;

        #[inline]
        fn from_str(s: &str) -> Result<Self, ParseError> {
            // this is very unsafe:
            // All Options and Results get unwrapped as we cannot create a ParseError

            let re = Regex::new(r"(.*)\s+(\w+)$").unwrap();
            let caps = re.captures(s).unwrap();

            let c1 = caps.get(1).map(|m| m.as_str()).unwrap();
            let c2 = caps.get(2).map(|m| m.as_str()).unwrap();

            let tz = c2.parse::<Tz>().unwrap();
            Tz::datetime_from_str(&tz, &c1, "%F %T").map(|dt| DateTime(dt))
        }
    }

    impl Parsable for DateTime<Tz> {
        #[inline]
        fn parse_from_str(s: &str, fmt: &str) -> Result<DateTime<Tz>, Error> {
            let re = Regex::new(r"(.*)\s+(\w+)$").unwrap();
            let caps = re.captures(s).ok_or(Error::BadFormat)?;

            let c1 = caps.get(1).map(|m| m.as_str()).ok_or(Error::BadFormat)?;
            let c2 = caps.get(2).map(|m| m.as_str()).ok_or(Error::BadFormat)?;

            let tz = c2.parse::<Tz>().map_err(|_| Error::BadFormat)?;
            Tz::datetime_from_str(&tz, &c1, fmt)
                .map(|dt| DateTime(dt))
                .map_err(|e| Error::Parsing(e))
        }
    }

    /// Parses a `Span` from a string in the format `{start} - {end}`.
    impl FromStr for DateTimeSpan<Tz> {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let re = Regex::new(r"(.*)\s+-\s+(.*)").unwrap();
            let caps = re.captures(s).ok_or(Error::Empty)?;

            let c1 = caps.get(1).ok_or(Error::NoStart)?;
            let c2 = caps.get(2).ok_or(Error::NoEnd)?;

            DateTimeSpan::new(
                DateTime::from_str(c1.as_str()).map(|dt| dt.0)?,
                DateTime::from_str(c2.as_str()).map(|dt| dt.0)?,
            )
        }
    }
}
