// duration - A simple cli to get the duration of the timespan.
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

use chrono::NaiveTime;
use timespan::NaiveTimeSpan;

fn usage() {
    println!(
        "Please provide exactly 4 arguments!

Usage: [span] [span_fmt] [start_fmt] [end_fmt]"
    );
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 5 {
        usage();
        std::process::exit(1);
    }

    args.next();
    let s = args.next().unwrap();
    let span_fmt = args.next().unwrap();
    let start_fmt = args.next().unwrap();
    let end_fmt = args.next().unwrap();

    let span = match NaiveTimeSpan::parse_from_str(
        s.as_str(),
        span_fmt.as_str(),
        start_fmt.as_str(),
        end_fmt.as_str(),
    ) {
        Ok(s) => s,
        Err(e) => {
            println!("An error occured: {}", e);
            std::process::exit(2);
        }
    };

    // chrono::Duration has no proper format method, so we use NaiveTime for formatting...
    let duration =
        NaiveTime::from_num_seconds_from_midnight(span.duration().num_seconds() as u32, 0);

    println!("duration for {}: {}", span, duration);
}
