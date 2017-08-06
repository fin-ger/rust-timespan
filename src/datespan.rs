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
use Spanable;
use Span;
use chrono::{DateTime, FixedOffset, Duration};
use regex;
use regex::Regex;

impl Spanable for DateTime<FixedOffset> {}

pub type Datespan = Span<DateTime<FixedOffset>>;

impl Span<DateTime<FixedOffset>> {
    pub fn parse_from_str(s: &str, fmt: &str, start: &str, end: &str) -> Result<Span<DateTime<FixedOffset>>, Error> {
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
                DateTime::parse_from_str(m1.as_str(), start)?,
                DateTime::parse_from_str(m2.as_str(), end)?,
            )
        } else {
            Span::new(
                DateTime::parse_from_str(m2.as_str(), start)?,
                DateTime::parse_from_str(m1.as_str(), end)?,
            )
        }
    }

    pub fn duration(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }
}
