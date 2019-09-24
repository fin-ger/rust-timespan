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

use crate::NaiveTimeSpan;
use std::fmt::Write;
use std::str::FromStr;

#[test]
fn fmt_test() {
    let ts = NaiveTimeSpan::from_str("09:00:00 - 17:00:00").unwrap();

    let d1 = ts.format("Opened from {start} to {end} on Monday", "%H.%M", "%H.%M");
    assert!(format!("{}", d1) == "Opened from 09.00 to 17.00 on Monday");

    let d2 = ts.format("Opened from {start}", "%H.%M", "");
    assert!(format!("{}", d2) == "Opened from 09.00");

    let d3 = ts.format("Opened until {end}", "", "%H.%M");
    assert!(format!("{}", d3) == "Opened until 17.00");

    let d4 = ts.format("{start} - {end}", "%Y", "%H.%M");
    let mut b1 = String::new();
    assert!(b1.write_fmt(format_args!("{}", d4)).is_err());

    let d5 = ts.format("{start} - {end}", "%H.%M", "%Y");
    let mut b2 = String::new();
    assert!(b2.write_fmt(format_args!("{}", d5)).is_err());
}
