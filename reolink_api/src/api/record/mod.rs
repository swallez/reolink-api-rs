use std::fmt::Formatter;
use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Unexpected};

pub mod download;
pub mod get_recording;
pub mod get_recording_v20;
pub mod nvr_download;
pub mod search;
pub mod snapshot;

#[derive(Clone, Debug)]
pub struct ScheduleTable(Vec<u8>);

/// A schedule table, with a single-digit value for every period in the table.
/// They're used mostly for boolean values, for which the docs states that "0" means "disabled".
/// We still use an `u8` for each entry, in case non-zero values are more than just "enabled"
/// (the doc shows examples with `2` without explaining how it's different from `1`).
impl Deref for ScheduleTable {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl <'de> Deserialize<'de> for ScheduleTable {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScheduleTable;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string composed of digits")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                let vec = v.as_bytes().iter()
                    .map(|b| {
                        if (b'0'..= b'9').contains(b) {
                            Ok(b - b'0')
                        } else {
                            Err(Error::invalid_value(Unexpected::Str(v), &self))
                        }
                    })
                    .collect::<Result<Vec<u8>, E>>()?;

                Ok(ScheduleTable(vec))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl Serialize for ScheduleTable {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s: String = self.0.iter().map(|v| char::from(*v + b'0')).collect();
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_schedule_table() -> anyhow::Result<()> {
        let json = r#""0102""#;
        let schedule = serde_json::from_str::<ScheduleTable>(json)?;
        assert_eq!(4, schedule.len());
        assert_eq!(&[0, 1, 0, 2], &schedule.as_slice());

        Ok(())
    }

    #[test]
    fn test_bad_schedule_table() {
        let json = r#""0abc""#;
        let result = serde_json::from_str::<ScheduleTable>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ser_schedule_table() -> anyhow::Result<()> {
        let table = ScheduleTable(vec![0, 1, 0, 2]);
        let json = serde_json::to_string(&table)?;
        assert_eq!(json, r#""0102""#);
        Ok(())
    }
}

//-------------------------------------------------------------------------------------------------

/// Date and time, expressed as a local date-time in the device's time zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {
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
    use crate::api::record::DateTime;

    impl From<NaiveDateTime> for DateTime {
        fn from(value: NaiveDateTime) -> Self {
            DateTime {
                year: value.year() as u16,
                mon: value.month() as u8,
                day: value.day() as u8,
                hour: value.hour() as u8,
                min: value.minute() as u8,
                sec: value.second() as u8,
            }
        }
    }

    impl From<DateTime> for NaiveDateTime {
        fn from(value: DateTime) -> Self {
            let day = NaiveDate::from_ymd_opt(value.year as i32, value.mon as u32, value.day as u32).expect("Invalid date");
            let time = NaiveTime::from_hms_opt(value.hour as u32, value.min as u32, value.sec as u32).expect("Invalid time");
            NaiveDateTime::new(day, time)
        }
    }

    #[cfg(test)]
    mod tests {
        //use crate::api::record::ScheduleTable;
        use super::*;

        #[test]
        fn test_into() {
            let ndt: NaiveDateTime = DateTime {
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
            let t: DateTime = ndt.into();

            assert_eq!(2024, t.year);
            assert_eq!(12, t.mon);
            assert_eq!(25, t.day);
            assert_eq!(1, t.hour);
            assert_eq!(2, t.min);
            assert_eq!(3, t.sec);
        }
    }
}
