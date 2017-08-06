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

use Span;
use std;
use chrono;

pub struct DelayedFormat<'a, T> {
    pub span: Span<T>,
    pub fmt: &'a str,
    pub start: &'a str,
    pub end: &'a str,
}

impl<'a> std::fmt::Display for DelayedFormat<'a, chrono::NaiveTime> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;

        // FIXME: this uses way too much memory

        let mut start_fmt = String::new();
        start_fmt.write_fmt(format_args!("{}", self.span.start.format(self.start)))?;
        start_fmt.shrink_to_fit();

        let mut end_fmt = String::new();
        end_fmt.write_fmt(format_args!("{}", self.span.end.format(self.end)))?;
        end_fmt.shrink_to_fit();

        let r1 = self.fmt.replace("{start}", start_fmt.as_str());
        let r2 = r1.replace("{end}", end_fmt.as_str());

        write!(f, "{}", r2)
    }
}
