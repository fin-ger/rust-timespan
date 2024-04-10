// timespan - A simple timespan for chrono times.
//
// Copyright (C) 2017
//     Fin Christensen <fin.christensen@posteo.de>
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

//! Date and time spans which do not concern about timezones.
//!
//! These time spans can be used for applications where the timezone is
//! irrelevant for the span.

mod naive_date_span;
mod naive_date_time_span;
mod naive_time_span;

pub use self::naive_date_span::NaiveDateSpan;
pub use self::naive_date_time_span::NaiveDateTimeSpan;
pub use self::naive_time_span::NaiveTimeSpan;
