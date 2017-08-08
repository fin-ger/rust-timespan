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

use NaiveTimeSpan;
use chrono::Duration;
use chrono::naive::NaiveTime;
use std::str::FromStr;

#[test]
fn new_test() {
    assert!(
        NaiveTimeSpan::new(
            NaiveTime::from_hms(12, 0, 0),
            NaiveTime::from_hms(12, 30, 0),
        ).is_ok()
    );

    assert!(
        NaiveTimeSpan::new(
            NaiveTime::from_hms(12, 30, 0),
            NaiveTime::from_hms(12, 0, 0),
        ).is_err()
    );
}

#[test]
fn parse_from_str_test() {
    let parse = NaiveTimeSpan::parse_from_str;
    let s = "from 09.00 to 17.00 on Monday";

    let ts1 = parse(s, "from {start} to {end} on Monday", "%H.%M", "%H.%M").unwrap();
    assert!(ts1 == NaiveTimeSpan::from_str("09:00:00 - 17:00:00").unwrap());

    let ts2 = parse("end: 17.00, start: 09.00", "end: {end}, start: {start}", "%H.%M", "%H.%M").unwrap();
    assert!(ts2 == NaiveTimeSpan::from_str("09:00:00 - 17:00:00").unwrap());

    assert!(parse(s, "foo", "%H.%M", "%H.%M").is_err()); // empty
    assert!(parse(s, "from {start}", "%H.%M", "%H.%M").is_err()); // no end
    assert!(parse(s, "to {end}", "%H.%M", "%H.%M").is_err()); // no start
    assert!(parse(s, "from {start} to {end} on Monday", "%Y", "%Y").is_err()); // wrong time format
}

#[test]
fn duration_test() {
    let ts = NaiveTimeSpan::from_str("13:00:00 - 14:00:00").unwrap();
    assert!(ts.duration() == Duration::hours(1));
}

#[test]
fn difference_test() {
    let t1  = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let t2  = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let t12 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t21 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    assert!(t1.difference(&t2).unwrap() == t12);
    assert!(t2.difference(&t1).unwrap() == t21);

    let t3 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    assert!(t3.difference(&t4).unwrap() == t3);
    assert!(t4.difference(&t3).unwrap() == t4);

    let t5 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t6 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    assert!(t5.difference(&t4).unwrap() == t5);
    assert!(t6.difference(&t3).unwrap() == t6);

    let t7 = NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap();
    let t8 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t9 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let ta = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    assert!(t7.difference(&t8).unwrap() == t2);
    assert!(t7.difference(&t9).is_err());
    assert!(t7.difference(&ta).unwrap() == t1);
    assert!(t8.difference(&t7).is_err());
    assert!(t9.difference(&t7).is_err());
    assert!(ta.difference(&t7).is_err());
}

#[test]
fn symmetric_difference_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();

    assert!(t1.symmetric_difference(&t2).unwrap() == t3);
    assert!(t2.symmetric_difference(&t1).unwrap() == t3);
    assert!(t1.symmetric_difference(&t4).is_err());
}

#[test]
fn intersection_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    let t5 = NaiveTimeSpan::from_str("12:00:00 - 13:00:00").unwrap();
    let t6 = NaiveTimeSpan::from_str("11:00:00 - 13:00:00").unwrap();

    assert!(t1.intersection(&t2).unwrap() == t2);
    assert!(t1.intersection(&t3).unwrap() == t3);
    assert!(t1.intersection(&t4).unwrap() == t4);
    assert!(t1.intersection(&t5).is_err());
    assert!(t1.intersection(&t6).unwrap() == t4);
}

#[test]
fn union_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("11:00:00 - 13:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("12:00:00 - 14:00:00").unwrap();

    let t12 = NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap();
    let t13 = NaiveTimeSpan::from_str("09:00:00 - 13:00:00").unwrap();

    assert!(t1.union(&t2).unwrap() == t12);
    assert!(t1.union(&t3).unwrap() == t13);
    assert!(t1.union(&t4).is_err());
}

#[test]
fn contains_test() {
    let ts = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t1 = NaiveTime::from_str("09:00:00").unwrap();
    let t2 = NaiveTime::from_str("09:30:00").unwrap();
    let t3 = NaiveTime::from_str("10:00:00").unwrap();
    let t4 = NaiveTime::from_str("10:30:00").unwrap();
    let t5 = NaiveTime::from_str("08:30:00").unwrap();

    assert!(ts.contains(&t1));
    assert!(ts.contains(&t2));
    assert!(ts.contains(&t3));
    assert!(!ts.contains(&t4));
    assert!(!ts.contains(&t5));
}

#[test]
fn is_disjoint_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("11:00:00 - 13:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("12:00:00 - 14:00:00").unwrap();

    assert!(!t1.is_disjoint(&t2));
    assert!(t1.is_disjoint(&t3));
    assert!(t1.is_disjoint(&t4));
}

#[test]
fn is_subset_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("08:00:00 - 09:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t5 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    let t6 = NaiveTimeSpan::from_str("12:00:00 - 13:00:00").unwrap();

    assert!(!t2.is_subset(&t1));
    assert!(t3.is_subset(&t1));
    assert!(t4.is_subset(&t1));
    assert!(t5.is_subset(&t1));
    assert!(!t6.is_subset(&t1));
}

#[test]
fn is_superset_test() {
    let t1 = NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("08:00:00 - 09:00:00").unwrap();
    let t3 = NaiveTimeSpan::from_str("09:00:00 - 10:00:00").unwrap();
    let t4 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t5 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();
    let t6 = NaiveTimeSpan::from_str("12:00:00 - 13:00:00").unwrap();

    assert!(!t1.is_superset(&t2));
    assert!(t1.is_superset(&t3));
    assert!(t1.is_superset(&t4));
    assert!(t1.is_superset(&t5));
    assert!(!t1.is_superset(&t6));
}

#[test]
fn split_off_test() {
    let ts = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let t1 = NaiveTime::from_str("09:00:00").unwrap();
    let t2 = NaiveTime::from_str("10:00:00").unwrap();
    let t3 = NaiveTime::from_str("11:00:00").unwrap();
    let t4 = NaiveTime::from_str("12:00:00").unwrap();
    let t5 = NaiveTime::from_str("13:00:00").unwrap();

    let ts1 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let ts2 = NaiveTimeSpan::from_str("11:00:00 - 12:00:00").unwrap();

    assert!(ts.split_off(&t1).is_err());
    assert!(ts.split_off(&t2).is_err());
    assert!(ts.split_off(&t3).unwrap() == (ts1, ts2));
    assert!(ts.split_off(&t4).is_err());
    assert!(ts.split_off(&t5).is_err());
}

#[test]
fn append_test() {
    let t1 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let d1 = Duration::hours(1);
    let d2 = Duration::hours(-1);

    let mut t3 = t1.clone();
    t3.append(&d1).unwrap();
    assert!(t3 == t2);

    let mut t4 = t1.clone();
    assert!(t4.append(&d2).is_err());
}

#[test]
fn prepend_test() {
    let t1 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let d1 = Duration::hours(1);
    let d2 = Duration::hours(-1);

    let mut t3 = t1.clone();
    t3.prepend(&d1).unwrap();
    assert!(t3 == t2);

    let mut t4 = t1.clone();
    assert!(t4.prepend(&d2).is_err());
}

#[test]
fn pop_test() {
    let t1 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("10:00:00 - 12:00:00").unwrap();
    let d1 = Duration::hours(1);
    let d2 = Duration::hours(-1);

    let mut t3 = t1.clone();
    assert!(t3.pop(&d1).is_err());

    let mut t4 = t1.clone();
    t4.pop(&d2).unwrap();
    assert!(t4 == t2);
}

#[test]
fn shift_test() {
    let t1 = NaiveTimeSpan::from_str("10:00:00 - 11:00:00").unwrap();
    let t2 = NaiveTimeSpan::from_str("09:00:00 - 11:00:00").unwrap();
    let d1 = Duration::hours(1);
    let d2 = Duration::hours(-1);

    let mut t3 = t1.clone();
    assert!(t3.shift(&d1).is_err());

    let mut t4 = t1.clone();
    t4.shift(&d2).unwrap();
    assert!(t4 == t2);
}

#[test]
fn from_str_test() {
    let parsed = NaiveTimeSpan::from_str("10:45:00 - 15:30:00").unwrap();
    let parsed_reference = NaiveTimeSpan::new(
        NaiveTime::from_hms(10, 45, 0),
        NaiveTime::from_hms(15, 30, 0),
    ).unwrap();
    assert!(parsed == parsed_reference);

    assert!(NaiveTimeSpan::from_str("10.45.00 - 15.30.00").is_err());
    assert!(NaiveTimeSpan::from_str("09:15:00-12:00:00").is_ok());
    assert!(NaiveTimeSpan::from_str("11:11").is_err());
    assert!(NaiveTimeSpan::from_str("").is_err());
}

#[test]
fn fmt_test() {
    let ts = NaiveTimeSpan::new(
        NaiveTime::from_hms(12, 0, 0),
        NaiveTime::from_hms(12, 30, 0),
    ).unwrap();

    assert!(format!("{}", ts) == "12:00:00 - 12:30:00");
}

#[cfg(feature = "with-serde")]
mod with_serde {
    use super::NaiveTimeSpan;
    use serde_json;
    use std::str::FromStr;

    #[derive(Serialize, Deserialize, PartialEq)]
    struct NaiveTimeSpanTest {
        pub span: NaiveTimeSpan,
    }

    #[test]
    fn serialize_test() {
        let reference =
            NaiveTimeSpanTest { span: NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap() };
        let json = r#"{"span":"09:00:00 - 12:00:00"}"#;
        assert!(serde_json::to_string(&reference).unwrap() == json);
    }

    #[test]
    fn deserialize_test() {
        let reference =
            NaiveTimeSpanTest { span: NaiveTimeSpan::from_str("09:00:00 - 12:00:00").unwrap() };
        let json = r#"{"span":"09:00:00 - 12:00:00"}"#;
        assert!(serde_json::from_str::<NaiveTimeSpanTest>(&json).unwrap() == reference);
    }
}
