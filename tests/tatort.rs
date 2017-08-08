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

extern crate timespan;
extern crate chrono;
extern crate chrono_tz;

use chrono_tz::Europe::Berlin;
use chrono::Utc;
use timespan::DateTimeSpan;

#[test]
fn tatort() {
    // support for something like "2017-08-03CEST - 2017-08-05CEST" is missing for now
    let _span = DateTimeSpan::from_utc_datetimespan(&"2017-04-02T20:15:00 - 2017-04-02T21:45:00".parse().unwrap(), &Berlin);

    // not working as chrono_tz::Tz is not implementing std::fmt::Display
    //assert!(span.contains(&"2017-04-02T20:37:21CEST".parse().unwrap()));
    //assert!(!span.contains(&"2017-04-02T22:12:19CEST".parse().unwrap()));

    let span = DateTimeSpan::from_utc_datetimespan(&"2017-04-02T20:15:00 - 2017-04-02T21:45:00".parse().unwrap(), &Utc);

    assert!(span.contains(&"2017-04-02T20:37:21+00:00".parse().unwrap()));
    assert!(!span.contains(&"2017-04-02T22:12:19+00:00".parse().unwrap()));
}
