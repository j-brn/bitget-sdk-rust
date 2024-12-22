use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn datetime_from_timestamp_str<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let ts = i64::from_str(s).map_err(D::Error::custom)?;

    Ok(DateTime::from_timestamp_nanos(ts))
}

pub(crate) fn datetime_from_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: i64 = Deserialize::deserialize(deserializer)?;

    Ok(DateTime::from_timestamp_nanos(ts))
}

pub(crate) fn parse_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr, <T as FromStr>::Err: std::fmt::Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let t = T::from_str(s).map_err(D::Error::custom)?;

    Ok(t)
}
