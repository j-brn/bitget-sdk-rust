use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub struct Candlestick {
    pub start_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume_coin: f64,
    pub volume_currency: f64,
}

impl<'de> Deserialize<'de> for Candlestick {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: [&str; 8] = Deserialize::deserialize(deserializer)?;
        let start_time = raw[0]
            .parse::<i64>()
            .map_err(D::Error::custom)
            .map(DateTime::from_timestamp_nanos)?;
        let open = raw[1].parse::<f64>().map_err(D::Error::custom)?;
        let high = raw[2].parse::<f64>().map_err(D::Error::custom)?;
        let low = raw[3].parse::<f64>().map_err(D::Error::custom)?;
        let close = raw[4].parse::<f64>().map_err(D::Error::custom)?;
        let volume_coin = raw[5].parse::<f64>().map_err(D::Error::custom)?;
        let volume_currency = raw[6].parse::<f64>().map_err(D::Error::custom)?;
        // status field seems to be useless as it seems to always match volume_currency so we
        // don't include it in the candlestick struct
        let _status = raw[6].parse::<f64>().map_err(D::Error::custom)?;

        Ok(Candlestick {
            start_time,
            open,
            high,
            low,
            close,
            volume_coin,
            volume_currency,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::future::public::candlestick::Candlestick;
    use crate::websocket::{DataPush, PushAction, SubscriptionArgs};
    use chrono::DateTime;

    #[test]
    pub fn test_deserialize_ws_push() {
        let s = r#"
            {
                "action": "snapshot",
                "arg": {
                    "instType": "USDT-FUTURES",
                    "channel": "candle1m",
                    "instId": "BTCUSDT"
                },
                "data": [
                    [
                        "1695685500000",
                        "27000",
                        "27000.5",
                        "27000",
                        "27000.5",
                        "0.057",
                        "1539.0155",
                        "1539.0155"
                    ]
                ],
                "ts": 1695715462250
            }
        "#;

        let expected = DataPush {
            action: PushAction::Snapshot,
            arg: SubscriptionArgs {
                inst_type: "USDT-FUTURES".to_owned(),
                channel: "candle1m".to_owned(),
                inst_id: "BTCUSDT".to_owned(),
            },
            data: vec![Candlestick {
                start_time: DateTime::from_timestamp_nanos(1695685500000),
                open: 27000.0,
                high: 27000.5,
                low: 27000.0,
                close: 27000.5,
                volume_coin: 0.057,
                volume_currency: 1539.0155,
            }],
            ts: DateTime::from_timestamp_nanos(1695715462250),
        };

        assert_eq!(
            serde_json::from_str::<DataPush<Candlestick>>(s).unwrap(),
            expected,
        )
    }
}
