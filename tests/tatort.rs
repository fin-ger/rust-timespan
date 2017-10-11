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

use chrono::offset::TimeZone;
use chrono_tz::Europe::Berlin;
use timespan::DateTimeSpan;

#[test]
fn tatort() {
    let span = DateTimeSpan::from_utc_datetimespan(
        &"2017-04-02T18:15:00 - 2017-04-02T19:45:00".parse().unwrap(),
        &Berlin,
    );

    assert!(span.contains(&Berlin.from_utc_datetime(
        &"2017-04-02T18:34:53".parse().unwrap(),
    )));
    assert!(!span.contains(&Berlin.from_utc_datetime(
        &"2017-04-03T20:12:19".parse().unwrap(),
    )));
    assert!(format!("{}", span) == "2017-04-02 20:15:00 CEST - 2017-04-02 21:45:00 CEST");
}
