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
use timespan::DateSpan;

#[test]
fn wacken() {
    // support for something like "2017-08-03CEST - 2017-08-05CEST" is missing for now
    let _span = DateSpan::from_utc_datespan(&"2017-08-03 - 2017-08-05".parse().unwrap(), &Berlin);

    // not working as chrono_tz::Tz is not implementing std::fmt::Display
    //assert!(span.contains(&"2017-08-04CEST".parse().unwrap()));
    //assert!(!span.contains(&"2017-07-16CEST".parse().unwrap()));

    // Date has no serialization and deserialization features, so it's not Spannable for now
    // A Span should be independent from serialization and deserialization as well
    // -> TODO
    //let span = DateSpan::from_utc_datespan(&"2017-08-03 - 2017-08-05".parse().unwrap(), &Utc);

    //assert!(span.contains(&"2017-08-04+00:00".parse().unwrap()));
    //assert!(!span.contains(&"2017-07-02+00:00".parse().unwrap()));
}
