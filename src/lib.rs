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

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(test)]
extern crate serde_json;

#[cfg(all(test, feature = "with-serde"))]
#[cfg_attr(all(test, feature = "with-serde"), macro_use)]
extern crate serde_derive;

extern crate chrono;
extern crate regex;

mod error;
mod delayed_format;
mod span;
mod traits;
mod date_span;
mod date_time_span;

pub mod naive;

#[cfg(test)]
mod span_test;
#[cfg(test)]
mod delayed_format_test;

pub use self::error::Error;
pub use self::delayed_format::DelayedFormat;
pub use self::span::Span;
pub use self::traits::Spanable;
pub use self::traits::Parsable;
pub use self::traits::Formatable;
pub use self::date_span::DateSpan;
pub use self::date_time_span::DateTimeSpan;
pub use self::naive::NaiveDateSpan;
pub use self::naive::NaiveDateTimeSpan;
pub use self::naive::NaiveTimeSpan;
