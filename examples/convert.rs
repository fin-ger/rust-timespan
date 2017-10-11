// convert - A simple cli to parse and format timespans.
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

use timespan::NaiveTimeSpan;

fn usage() {
    println!(
        "Please provide exactly 7 arguments!

Usage: [span]
       [from_span_fmt] [from_start_fmt] [from_end_fmt]
       [to_span_fmt] [to_start_fmt] [to_end_fmt]"
    );
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 8 {
        usage();
        std::process::exit(1);
    }

    args.next();
    let s = args.next().unwrap();
    let from_span_fmt = args.next().unwrap();
    let from_start_fmt = args.next().unwrap();
    let from_end_fmt = args.next().unwrap();
    let to_span_fmt = args.next().unwrap();
    let to_start_fmt = args.next().unwrap();
    let to_end_fmt = args.next().unwrap();

    let span = match NaiveTimeSpan::parse_from_str(
        s.as_str(),
        from_span_fmt.as_str(),
        from_start_fmt.as_str(),
        from_end_fmt.as_str(),
    ) {
        Ok(s) => s,
        Err(e) => {
            println!("An error occured: {}", e);
            std::process::exit(2);
        }
    };
    println!(
        "{}",
        span.format(
            to_span_fmt.as_str(),
            to_start_fmt.as_str(),
            to_end_fmt.as_str(),
        )
    );
}
