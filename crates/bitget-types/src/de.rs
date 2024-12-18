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

pub(crate) fn parse_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let num = f64::from_str(s).map_err(D::Error::custom)?;

    Ok(num)
}

pub(crate) fn parse_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let num = usize::from_str(s).map_err(D::Error::custom)?;

    Ok(num)
}
