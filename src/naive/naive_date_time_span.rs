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

use Spanable;
use ParseFromStr;
use Span;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{ParseResult, NaiveDateTime, Duration};

impl Spanable for NaiveDateTime {
    #[inline]
    fn format<'a>(&self, fmt: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        NaiveDateTime::format(self, fmt)
    }

    #[inline]
    fn signed_duration_since(self, other: Self) -> Duration {
        NaiveDateTime::signed_duration_since(self, other)
    }
}

impl ParseFromStr for NaiveDateTime {
    #[inline]
    fn parse_from_str(s: &str, fmt: &str) -> ParseResult<Self> {
        NaiveDateTime::parse_from_str(s, fmt)
    }
}

pub type NaiveDateTimeSpan = Span<NaiveDateTime>;
