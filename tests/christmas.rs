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

extern crate timespan;

use timespan::NaiveDateSpan;

#[test]
fn naive_date_span_christmas() {
    let span: NaiveDateSpan = "2017-12-24 - 2017-12-26".parse().unwrap();

    assert!(span.start == "2017-12-24".parse().unwrap());
    assert!(span.end == "2017-12-26".parse().unwrap());
    assert!(span.contains(&"2017-12-25".parse().unwrap()));
}
