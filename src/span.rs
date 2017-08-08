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
use DelayedFormat;
use Spanable;
use ParseFromStr;
use chrono::Duration;
use regex;
use regex::Regex;
use std;

#[derive(PartialEq, Clone)]
pub struct Span<T> {
    pub start: T,
    pub end: T,
}

impl<T> Span<T> where T: Spanable {
    pub fn new(start: T, end: T) -> Result<Span<T>, Error> {
        if start >= end {
            return Err(Error::Ordering);
        }

        Ok(Span {
            start: start,
            end: end,
        })
    }

    pub fn format<'a>(&self, fmt: &'a str, start: &'a str, end: &'a str) -> DelayedFormat<'a, T> {
        DelayedFormat {
            span: self.clone(),
            fmt: fmt,
            start: start,
            end: end,
        }
    }

    pub fn duration(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }

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
            return Ok(Span { start: self.start, end: other.start });
        } else if self.start >= other.start && self.start < other.end && self.end > other.end {
            // -(--[-)##]-
            return Ok(Span { start: other.end, end: self.end });
        } else {
            // -[##(-)##]- -> err
            return Err(Error::NotContinuous);
        }
    }

    pub fn symmetric_difference(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.end == other.start {
            // -[##](##)-
            return Ok(Span { start: self.start, end: other.end });
        } else if other.end == self.start {
            // -(##)[##]-
            return Ok(Span { start: other.start, end: self.end });
        } else {
            return Err(Error::NotContinuous);
        }
    }

    pub fn intersection(&self, other: &Span<T>) -> Result<Span<T>, Error> {
        if self.end == other.start {
            Err(Error::Empty)
        } else if self.end < other.start || other.end < self.start {
            Err(Error::NotContinuous)
        } else {
            Ok(Span {
                start: std::cmp::max(self.start, other.start),
                end: std::cmp::min(self.end, other.end)
            })
        }
    }

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

    pub fn contains(&self, item: &T) -> bool {
        self.start <= *item && self.end >= *item
    }

    pub fn is_disjoint(&self, other: &Span<T>) -> bool {
        self.end <= other.start || self.start >= other.end
    }

    pub fn is_subset(&self, other: &Span<T>) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn is_superset(&self, other: &Span<T>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn split_off(&self, at: &T) -> Result<(Span<T>, Span<T>), Error> {
        if self.start >= *at || self.end <= *at {
            return Err(Error::OutOfRange);
        }

        Ok((
            Span { start: self.start, end: *at },
            Span { start: *at, end: self.end },
        ))
    }

    pub fn append(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.end + *time;
        if new <= self.start {
            return Err(Error::Empty);
        }
        self.end = new;
        Ok(())
    }

    pub fn prepend(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.start - *time;
        if new >= self.end {
            return Err(Error::Empty);
        }
        self.start = new;
        Ok(())
    }

    pub fn pop(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.end - *time;
        if new <= self.start {
            return Err(Error::Empty);
        }
        self.end = new;
        Ok(())
    }

    pub fn shift(&mut self, time: &Duration) -> Result<(), Error> {
        let new = self.start + *time;
        if new >= self.end {
            return Err(Error::Empty);
        }
        self.start = new;
        Ok(())
    }
}

impl<T> Span<T> where T: Spanable + ParseFromStr {
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

impl<T> std::str::FromStr for Span<T> where T: Spanable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(.*)\s+-\s+(.*)").unwrap();
        let caps = re.captures(s).ok_or(Error::Empty)?;

        let c1 = caps.get(1).ok_or(Error::NoStart)?;
        let c2 = caps.get(2).ok_or(Error::NoEnd)?;

        Span::new(
            T::from_str(c1.as_str())?,
            T::from_str(c2.as_str())?,
        )
    }
}

impl<T> std::fmt::Debug for Span<T> where T: Spanable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl<T> std::fmt::Display for Span<T> where T: Spanable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "with-serde")]
mod with_serde {
    use super::Span;
    use super::Spanable;
    use serde::{de, ser};
    use std::marker::PhantomData;
    use std::fmt;

    impl<T> ser::Serialize for Span<T> where T: Spanable {
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

    impl<'de, T> de::Visitor<'de> for SpanVisitor<T> where T: Spanable {
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

    impl<'de, T> de::Deserialize<'de> for Span<T> where T: Spanable {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(SpanVisitor { phantom: PhantomData })
        }
    }
}
