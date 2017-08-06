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

use TimespanError;
use DelayedFormat;
use chrono::Duration;
use chrono::naive::NaiveTime;
use regex;
use regex::Regex;
use std;

#[derive(PartialEq, Clone)]
pub struct Timespan {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl Timespan {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Result<Timespan, TimespanError> {
        if start >= end {
            return Err(TimespanError::Ordering);
        }

        Ok(Timespan {
            start: start,
            end: end,
        })
    }

    pub fn parse_from_str(s: &str, fmt: &str, start: &str, end: &str) -> Result<Timespan, TimespanError> {
        let esc = regex::escape(fmt);

        let repl_re = Regex::new(r"(?:\\\{start\\\}|\\\{end\\\})").unwrap();
        let repl = repl_re.replace_all(&esc, r"(.*)");

        let re = Regex::new(&repl)?;
        let caps = re.captures(s).ok_or(TimespanError::Empty)?;

        let start_idx = fmt.find("{start}").ok_or(TimespanError::NoStart)?;
        let end_idx = fmt.find("{end}").ok_or(TimespanError::NoEnd)?;

        // we already checked for the existance of {start} and {end} captures -> unwrap allowed
        let m1 = caps.get(1).unwrap();
        let m2 = caps.get(2).unwrap();

        if start_idx < end_idx {
            Timespan::new(
                NaiveTime::parse_from_str(m1.as_str(), start)?,
                NaiveTime::parse_from_str(m2.as_str(), end)?,
            )
        } else {
            Timespan::new(
                NaiveTime::parse_from_str(m2.as_str(), start)?,
                NaiveTime::parse_from_str(m1.as_str(), end)?,
            )
        }
    }

    pub fn format<'a>(&self, fmt: &'a str, start: &'a str, end: &'a str) -> DelayedFormat<'a> {
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

    pub fn difference(&self, other: &Timespan) -> Result<Timespan, TimespanError> {
        if self.start >= other.start && self.end <= other.end {
            // -(--[-]--)- -> err
            return Err(TimespanError::Empty);
        } else if self.end <= other.start {
            // -[##]-(--)-
            return Ok(self.clone());
        } else if self.start >= other.end {
            // -(--)-[##]-
            return Ok(self.clone());
        } else if self.end > other.start && self.end <= other.end && self.start < other.start {
            // -[##(-]--)-
            return Ok(Timespan { start: self.start, end: other.start });
        } else if self.start >= other.start && self.start < other.end && self.end > other.end {
            // -(--[-)##]-
            return Ok(Timespan { start: other.end, end: self.end });
        } else {
            // -[##(-)##]- -> err
            return Err(TimespanError::NotContinuous);
        }
    }

    pub fn symmetric_difference(&self, other: &Timespan) -> Result<Timespan, TimespanError> {
        if self.end == other.start {
            // -[##](##)-
            return Ok(Timespan { start: self.start, end: other.end });
        } else if other.end == self.start {
            // -(##)[##]-
            return Ok(Timespan { start: other.start, end: self.end });
        } else {
            return Err(TimespanError::NotContinuous);
        }
    }

    pub fn intersection(&self, other: &Timespan) -> Result<Timespan, TimespanError> {
        if self.end == other.start {
            Err(TimespanError::Empty)
        } else if self.end < other.start || other.end < self.start {
            Err(TimespanError::NotContinuous)
        } else {
            Ok(Timespan {
                start: std::cmp::max(self.start, other.start),
                end: std::cmp::min(self.end, other.end)
            })
        }
    }

    pub fn union(&self, other: &Timespan) -> Result<Timespan, TimespanError> {
        if self.end < other.start || other.end < self.start {
            Err(TimespanError::NotContinuous)
        } else {
            Ok(Timespan {
                start: std::cmp::min(self.start, other.start),
                end: std::cmp::max(self.end, other.end),
            })
        }
    }

    pub fn contains(&self, item: &NaiveTime) -> bool {
        self.start <= *item && self.end >= *item
    }

    pub fn is_disjoint(&self, other: &Timespan) -> bool {
        self.end <= other.start || self.start >= other.end
    }

    pub fn is_subset(&self, other: &Timespan) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn is_superset(&self, other: &Timespan) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn split_off(&self, at: &NaiveTime) -> Result<(Timespan, Timespan), TimespanError> {
        if self.start >= *at || self.end <= *at {
            return Err(TimespanError::OutOfRange);
        }

        Ok((
            Timespan { start: self.start, end: *at },
            Timespan { start: *at, end: self.end },
        ))
    }

    pub fn append(&mut self, time: &Duration) -> Result<(), TimespanError> {
        let new = self.end + *time;
        if new <= self.start {
            return Err(TimespanError::Empty);
        }
        self.end = new;
        Ok(())
    }

    pub fn prepend(&mut self, time: &Duration) -> Result<(), TimespanError> {
        let new = self.start - *time;
        if new >= self.end {
            return Err(TimespanError::Empty);
        }
        self.start = new;
        Ok(())
    }

    pub fn pop(&mut self, time: &Duration) -> Result<(), TimespanError> {
        let new = self.end - *time;
        if new <= self.start {
            return Err(TimespanError::Empty);
        }
        self.end = new;
        Ok(())
    }

    pub fn shift(&mut self, time: &Duration) -> Result<(), TimespanError> {
        let new = self.start + *time;
        if new >= self.end {
            return Err(TimespanError::Empty);
        }
        self.start = new;
        Ok(())
    }
}

impl std::str::FromStr for Timespan {
    type Err = TimespanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\s*-\s*").unwrap();
        let mut span: Vec<&str> = re.splitn(s, 2).collect();

        // make sure span[1], etc. do not go out of bounds.
        while span.len() < 2 {
            span.push("");
        }

        Timespan::new(
            NaiveTime::from_str(span[0])?,
            NaiveTime::from_str(span[1])?,
        )
    }
}

impl std::fmt::Debug for Timespan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl std::fmt::Display for Timespan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "with-serde")]
mod with_serde {
    use super::Timespan;
    use serde::{de, ser};
    use std::fmt;

    impl ser::Serialize for Timespan {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.collect_str(&self)
        }
    }

    struct TimespanVisitor;

    impl<'de> de::Visitor<'de> for TimespanVisitor {
        type Value = Timespan;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a formatted time span string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Timespan, E>
        where
            E: de::Error,
        {
            value.parse().map_err(|err| E::custom(format!("{}", err)))
        }
    }

    impl<'de> de::Deserialize<'de> for Timespan {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(TimespanVisitor)
        }
    }
}
