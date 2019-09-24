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

use crate::DelayedFormat;
use crate::Error;
use crate::Formatable;
use crate::Parsable;
use crate::Spanable;
use chrono::Duration;
use regex;
use regex::Regex;
use std;

/// This describes a span of something that is `Spanable` by providing a start and end point.
///
/// When the provided `Spanable` type `T` is `Formatable` the span can be serialized to
/// a string. For deserialization from a string the `Parsable` trait must be implemented by `T`.
/// Support for `serde` is available when the `timespan` crate is configured with the
/// `with-serde` feature.
///
/// This type implements operations known from the set theory. However, there are only operations
/// allowed that produce a single span (e.g. the resulting span is continuous) that must not be
/// empty. When an operation would violate these restrictions an error is emitted.
///
/// > Developer note: A `Span` may accept all possible input values without leading to errors in
/// > the future by producing an iterator over the results allowing an arbitrary amount of
/// > resulting spans.
///
/// # Example
///
/// ~~~~
/// # extern crate timespan; extern crate chrono; fn main() {
/// use timespan::Span;
/// use chrono::NaiveTime;
///
/// let start = "12:30:00".parse().unwrap();
/// let end = "14:45:00".parse().unwrap();
/// let span: Span<NaiveTime> = Span::new(start, end).unwrap();
///
/// assert!(format!("{}", span) == "12:30:00 - 14:45:00");
/// # }
/// ~~~~
#[derive(PartialEq, Clone)]
pub struct Span<T> {
    /// The starting point of the span.
    pub start: T,
    /// The end point of the span.
    pub end: T,
}

impl<T> Span<T>
where
    T: Spanable,
{
    /// Create a new span with a given starting point and a given end point.
    ///
    /// This method emits an `Error::Ordering` error when the end point lies
    /// before the start point.
    pub fn new(start: T, end: T) -> Result<Span<T>, Error> {
        if start >= end {
            return Err(Error::Ordering);
        }

        Ok(Span {
            start: start,
            end: end,
        })
    }

    /// Get the total duration of the span as a `chrono::Duration`.
    pub fn duration(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }

    /// Calculate the mathematical difference of two spans with the same `Spanable` type.
    ///
    /// The difference of span `self` and `other` includes the parts of span `self` that are
    /// not included in span `other`.
    ///
    /// This method produces an error when
    ///
    ///  - the resulting difference would produce an empty span (`Error::Empty`)
    ///  - the resulting difference is not continuous (e.g. splitted) (`Error::NotContinuous`)
    ///
    pub fn difference(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.start >= other.start && self.end <= other.end {
            // -(--[-]--)- -> err
            return Err(Error::Empty);
        } else if self.end <= other.start {
            // -[##]-(--)-
            return Ok(self.clone());
        } else if self.start >= other.end {
            // -(--)-[##]-
            return Ok(self.clone());
        } else if self.end > other.start && self.end <= other.end && self.start < other.start {
            // -[##(-]--)-
            return Ok(Span {
                start: self.start,
                end: other.start,
            });
        } else if self.start >= other.start && self.start < other.end && self.end > other.end {
            // -(--[-)##]-
            return Ok(Span {
                start: other.end,
                end: self.end,
            });
        } else {
            // -[##(-)##]- -> err
            return Err(Error::NotContinuous);
        }
    }

    /// Calculate the mathematical symmetric difference of two spans with the same `Spanable` type.
    ///
    /// The symmetric difference of span `self` and `other` includes the parts of span `self` that
    /// are not included in span `other` and the parts of span `other` that are not included in span
    /// `self`.
    ///
    /// This method produces an error when the resulting symmetric difference is not continuous
    /// (e.g. splitted) (`Error::NotContinuous`). As this is only not the case when the two spans
    /// are adjacent this method will most likely produce an error.
    pub fn symmetric_difference(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.end == other.start {
            // -[##](##)-
            return Ok(Span {
                start: self.start,
                end: other.end,
            });
        } else if other.end == self.start {
            // -(##)[##]-
            return Ok(Span {
                start: other.start,
                end: self.end,
            });
        } else {
            return Err(Error::NotContinuous);
        }
    }

    /// Calculate the mathematical intersection of two spans with the same `Spanable` type.
    ///
    /// The intersection of span `self` and `other` includes the parts that are included in span `self`
    /// and span `other`.
    ///
    /// This method produces an `Error::Empty` error when there is no intersection between the
    /// two spans.
    pub fn intersection(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.end <= other.start || other.end <= self.start {
            Err(Error::Empty)
        } else {
            Ok(Span {
                start: std::cmp::max(self.start, other.start),
                end: std::cmp::min(self.end, other.end),
            })
        }
    }

    /// Calculate the mathematical union of two spans with the same `Spanable` type.
    ///
    /// The union of span `self` and `other` includes the parts that are included in span `self` or
    /// span `other`.
    ///
    /// This method produces an `Error::NotContinuous` error when the two spans are not intersecting
    /// or adjacent to each other.
    pub fn union(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.end < other.start || other.end < self.start {
            Err(Error::NotContinuous)
        } else {
            Ok(Span {
                start: std::cmp::min(self.start, other.start),
                end: std::cmp::max(self.end, other.end),
            })
        }
    }

    /// Returns `true` when a given `Spanable` is included in `self`. Otherwise returns `false`.
    pub fn contains(&self, item: &T) -> bool {
        self.start <= *item && self.end >= *item
    }

    /// Returns `true` when `self` has no parts in common with `other`. Otherwise returns `false`.
    pub fn is_disjoint(&self, other: &Span<T>) -> bool {
        self.end <= other.start || self.start >= other.end
    }

    /// Returns `true` when `self` is completely included in `other`. Otherwise returns `false`.
    pub fn is_subset(&self, other: &Span<T>) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    /// Returns `true` when `other` is completely included in `self`. Otherwise returns `false`.
    pub fn is_superset(&self, other: &Span<T>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Split `self` at a given time point `at` into two spans of the same `Spanable` type.
    ///
    /// This emits an `Error::OutOfRange` error when `at` is not included in `self`.
    pub fn split_off(&self, at: &T) -> Result<(Span<T>, Span<T>), Error> {
        if self.start >= *at || self.end <= *at {
            return Err(Error::OutOfRange);
        }

        Ok((
            Span {
                start: self.start,
                end: *at,
            },
            Span {
                start: *at,
                end: self.end,
            },
        ))
    }

    /// Move the end point forward in time by a given duration.
    ///
    /// This emits an `Error::Empty` error when the operation would produce an empty span
    /// (e.g. the duration is negative).
    pub fn append(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.end + *time;
        if new <= self.start {
            return Err(Error::Empty);
        }
        self.end = new;
        Ok(())
    }

    /// Move the start point backward in time by a given duration.
    ///
    /// This emits an `Error::Empty` error when the operation would produce an empty span.
    /// (e.g. the duration is negative).
    pub fn prepend(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.start - *time;
        if new >= self.end {
            return Err(Error::Empty);
        }
        self.start = new;
        Ok(())
    }

    /// Move the end point backward in time by a given duration.
    ///
    /// This emits an `Error::Empty` error when the operation would produce an empty span.
    pub fn pop(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.end - *time;
        if new <= self.start {
            return Err(Error::Empty);
        }
        self.end = new;
        Ok(())
    }

    /// Move the start point forward in time by a given duration.
    ///
    /// This emits an `Error::Empty` error when the operation would produce an empty span.
    pub fn shift(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.start + *time;
        if new >= self.end {
            return Err(Error::Empty);
        }
        self.start = new;
        Ok(())
    }
}

impl<T> Span<T>
where
    T: Spanable + Formatable,
{
    /// Formats the span with the specified format strings.
    ///
    /// For the `start` and `end` format strings see the `chrono::format::strftime` module.
    ///
    /// The `fmt` string is used to format the span to a string. It must contain the following
    /// substrings:
    ///
    ///  - `{start}` to match the `start` point of the span
    ///  - `{end}` to match the `end` point of the span
    ///
    /// # Example
    ///
    /// ~~~~
    /// # extern crate timespan; fn main() {
    /// use timespan::NaiveTimeSpan;
    ///
    /// let span: NaiveTimeSpan = "12:30:00 - 14:45:00".parse().unwrap();
    ///
    /// let f = span.format("from {start} to {end}", "%H.%M", "%H.%M");
    /// assert!(f.to_string() == "from 12.30 to 14.45");
    /// assert!(format!("{}", f) == "from 12.30 to 14.45");
    /// # }
    /// ~~~~
    pub fn format<'a>(&self, fmt: &'a str, start: &'a str, end: &'a str) -> DelayedFormat<'a, T> {
        DelayedFormat {
            span: self.clone(),
            fmt: fmt,
            start: start,
            end: end,
        }
    }
}

impl<T> Span<T>
where
    T: Spanable + Parsable,
{
    /// Parses the span with the specified format strings from a given string `s`.
    ///
    /// For the `start` and `end` format strings see the `chrono::format::strftime` module.
    ///
    /// The `fmt` string is used to parse a span from a string. It must contain the following
    /// substrings:
    ///
    ///  - `{start}` to match the `start` point of the span
    ///  - `{end}` to match the `end` point of the span
    ///
    /// # Example
    /// ~~~~
    /// # extern crate timespan; fn main() {
    /// use timespan::NaiveTimeSpan;
    ///
    /// let span = NaiveTimeSpan::parse_from_str(
    ///     "from 12.30 to 14.45",
    ///     "from {start} to {end}",
    ///     "%H.%M",
    ///     "%H.%M",
    /// ).unwrap();
    ///
    /// assert!(format!("{}", span) == "12:30:00 - 14:45:00");
    /// # }
    /// ~~~~
    pub fn parse_from_str(s: &str, fmt: &str, start: &str, end: &str) -> Result<Span<T>, Error> {
        let esc = regex::escape(fmt);

        let repl_re = Regex::new(r"(?:\\\{start\\\}|\\\{end\\\})").unwrap();
        let repl = repl_re.replace_all(&esc, r"(.*)");

        let re = Regex::new(&repl)?;
        let caps = re.captures(s).ok_or(Error::Empty)?;

        let start_idx = fmt.find("{start}").ok_or(Error::NoStart)?;
        let end_idx = fmt.find("{end}").ok_or(Error::NoEnd)?;

        // we already checked for the existance of {start} and {end} captures -> unwrap allowed
        let m1 = caps.get(1).unwrap();
        let m2 = caps.get(2).unwrap();

        if start_idx < end_idx {
            Span::new(
                T::parse_from_str(m1.as_str(), start)?,
                T::parse_from_str(m2.as_str(), end)?,
            )
        } else {
            Span::new(
                T::parse_from_str(m2.as_str(), start)?,
                T::parse_from_str(m1.as_str(), end)?,
            )
        }
    }
}

/// Parses a `Span` from a string in the format `{start} - {end}`.
impl<T> std::str::FromStr for Span<T>
where
    T: Spanable + Parsable,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(.*)\s+-\s+(.*)").unwrap();
        let caps = re.captures(s).ok_or(Error::Empty)?;

        let c1 = caps.get(1).ok_or(Error::NoStart)?;
        let c2 = caps.get(2).ok_or(Error::NoEnd)?;

        Span::new(T::from_str(c1.as_str())?, T::from_str(c2.as_str())?)
    }
}

/// Formats a `Span` in the format `{start} - {end}`.
impl<T> std::fmt::Debug for Span<T>
where
    T: Spanable + Formatable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

/// Formats a `Span` in the format `{start} - {end}`.
impl<T> std::fmt::Display for Span<T>
where
    T: Spanable + Formatable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "with-serde")]
mod with_serde {
    use super::Formatable;
    use super::Parsable;
    use super::Span;
    use super::Spanable;
    use serde::{de, ser};
    use std::fmt;
    use std::marker::PhantomData;

    impl<T> ser::Serialize for Span<T>
    where
        T: Spanable + Formatable,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.collect_str(&self)
        }
    }

    struct SpanVisitor<T> {
        phantom: PhantomData<T>,
    }

    impl<'de, T> de::Visitor<'de> for SpanVisitor<T>
    where
        T: Spanable + Parsable,
    {
        type Value = Span<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a formatted time span string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Span<T>, E>
        where
            E: de::Error,
        {
            value.parse().map_err(|err| E::custom(format!("{}", err)))
        }
    }

    impl<'de, T> de::Deserialize<'de> for Span<T>
    where
        T: Spanable + Parsable,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(SpanVisitor {
                phantom: PhantomData,
            })
        }
    }
}
