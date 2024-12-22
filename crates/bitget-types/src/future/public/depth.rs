use crate::de::{datetime_from_timestamp_str, parse_from_str};
use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{format, write, Display, Formatter};

#[derive(Debug, PartialEq)]
struct Quote {
    price: f64,
    size: f64,
}

impl<'de> Deserialize<'de> for Quote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: [&str; 2] = Deserialize::deserialize(deserializer)?;
        let price = raw[0].parse::<f64>().map_err(D::Error::custom)?;
        let size = raw[1].parse::<f64>().map_err(D::Error::custom)?;

        Ok(Quote { price, size })
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct OrderBook {
    asks: Vec<Quote>,
    bids: Vec<Quote>,
    checksum: u32,
    #[serde(deserialize_with = "datetime_from_timestamp_str")]
    ts: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use crate::future::public::depth::{OrderBook, Quote};
    use crate::websocket::{DataPush, PushAction, SubscriptionArgs};
    use chrono::DateTime;

    #[test]
    pub fn test_deserialize_data_push() {
        let s = r#"
            {
                "action": "snapshot",
                "arg": {
                    "instType": "USDT-FUTURES",
                    "channel": "books5",
                    "instId": "BTCUSDT"
                },
                "data": [
                    {
                        "asks": [
                            [
                                "27000.5",
                                "8.760"
                            ],
                            [
                                "27001.0",
                                "0.400"
                            ]
                        ],
                        "bids": [
                            [
                                "27000.0",
                                "2.710"
                            ],
                            [
                                "26999.5",
                                "1.460"
                            ]
                        ],
                        "checksum": 0,
                        "ts": "1695716059516"
                    }
                ],
                "ts": 1695716059516
            }
        "#;

        let expected = DataPush {
            action: PushAction::Snapshot,
            arg: SubscriptionArgs {
                inst_type: "USDT-FUTURES".to_owned(),
                channel: "books5".to_owned(),
                inst_id: "BTCUSDT".to_owned(),
            },
            data: vec![OrderBook {
                asks: vec![
                    Quote {
                        price: 27000.5,
                        size: 8.760,
                    },
                    Quote {
                        price: 27001.0,
                        size: 0.400,
                    },
                ],
                bids: vec![
                    Quote {
                        price: 27000.0,
                        size: 2.710,
                    },
                    Quote {
                        price: 26999.5,
                        size: 1.460,
                    },
                ],
                checksum: 0,
                ts: DateTime::from_timestamp_nanos(1695716059516),
            }],
            ts: DateTime::from_timestamp_nanos(1695716059516),
        };

        assert_eq!(
            serde_json::from_str::<DataPush<OrderBook>>(s).unwrap(),
            expected
        )
    }
}
