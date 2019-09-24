// contains - A simple cli to check whether a given point in time is
//            included in a given timespan.
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

extern crate chrono;
extern crate timespan;

use chrono::NaiveTime;
use timespan::NaiveTimeSpan;

fn usage() {
    println!(
        "Please provide exactly 6 arguments!

Usage: [span] [point] [span_fmt] [start_fmt] [end_fmt] [point_fmt]"
    );
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 7 {
        usage();
        std::process::exit(1);
    }

    args.next();
    let s = args.next().unwrap();
    let p = args.next().unwrap();
    let span_fmt = args.next().unwrap();
    let start_fmt = args.next().unwrap();
    let end_fmt = args.next().unwrap();
    let p_fmt = args.next().unwrap();

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

    let point = NaiveTime::parse_from_str(p.as_str(), p_fmt.as_str()).unwrap();
    let c = span.contains(&point);

    println!(
        "{} {} {}",
        span,
        if c { "contains" } else { "does not contain" },
        point
    );

    std::process::exit(!c as i32);
}
