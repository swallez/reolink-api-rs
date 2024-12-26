use serde::{Deserialize, Serialize};

pub mod get_recording;
pub mod get_recording_v20;
pub mod search;
pub mod download;
pub mod snapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub year: u16,
    pub mon: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

#[cfg(feature = "chrono")]
mod chrono_impl {
    use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
    use crate::api::record::Time;

    impl From<NaiveDateTime> for Time {
        fn from(value: NaiveDateTime) -> Self {
            Time {
                year: value.year() as u16,
                mon: value.month() as u8,
                day: value.day() as u8,
                hour: value.hour() as u8,
                min: value.minute() as u8,
                sec: value.second() as u8,
            }
        }
    }

    impl From<Time> for NaiveDateTime {
        fn from(value: Time) -> Self {
            let day = NaiveDate::from_ymd_opt(value.year as i32, value.mon as u32, value.day as u32).expect("Invalid date");
            let time = NaiveTime::from_hms_opt(value.hour as u32, value.min as u32, value.sec as u32).expect("Invalid time");
            NaiveDateTime::new(day, time)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_into() {
            let ndt: NaiveDateTime = Time {
                year: 2024,
                mon: 12,
                day: 25,
                hour: 1,
                min: 2,
                sec: 3,
            }.into();

            assert_eq!(2024, ndt.year());
            assert_eq!(12, ndt.month());
            assert_eq!(25, ndt.day());
            assert_eq!(1, ndt.hour());
            assert_eq!(2, ndt.minute());
            assert_eq!(3, ndt.second());
        }

        #[test]
        fn test_from() {
            let ndt = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap().and_hms_opt(1, 2, 3).unwrap();
            let t: Time = ndt.into();

            assert_eq!(2024, t.year);
            assert_eq!(12, t.mon);
            assert_eq!(25, t.day);
            assert_eq!(1, t.hour);
            assert_eq!(2, t.min);
            assert_eq!(3, t.sec);
        }
    }
}
