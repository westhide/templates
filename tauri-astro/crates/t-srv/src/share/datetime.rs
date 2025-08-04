use chrono::{DateTime, FixedOffset, MappedLocalTime, NaiveDate, TimeZone, offset::LocalResult};

use crate::error::{Result, err};

pub const TZ: FixedOffset = FixedOffset::east_opt(0 * 3600).expect("UTC");

pub trait Datetime<T>
where
    T: TimeZone,
{
    fn into_date(self) -> Result<DateTime<T>>;
    fn into_time(self) -> Result<u64>;
}

impl<T> Datetime<T> for MappedLocalTime<DateTime<T>>
where
    T: TimeZone,
{
    fn into_date(self) -> Result<DateTime<T>> {
        match self {
            LocalResult::Single(date) => Ok(date),
            local_resutl => err!("Invalid Date: {local_resutl:?}"),
        }
    }

    fn into_time(self) -> Result<u64> {
        let time = self.into_date()?.timestamp() as u64;
        Ok(time)
    }
}

pub fn parse_date(s: &str) -> Result<NaiveDate> {
    let local = NaiveDate::parse_from_str(&s, "%Y-%m-%d")?;
    Ok(local)
}

pub fn get_date_time(date: &NaiveDate, hour: u32, min: u32, sec: u32) -> Result<u64> {
    match date.and_hms_opt(hour, min, sec) {
        Some(local) => TZ.from_local_datetime(&local).into_time(),
        None => err!("Invalid Datetime HMS"),
    }
}
