use crate::de::{datetime_from_timestamp_str, parse_from_str};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSide {
    Sell,
    Buy,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(deserialize_with = "datetime_from_timestamp_str")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "parse_from_str")]
    pub price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub size: f64,
    pub side: TradeSide,
    #[serde(deserialize_with = "parse_from_str")]
    pub trade_id: usize,
}

#[cfg(test)]
mod tests {
    use super::{Trade, TradeSide};
    use crate::websocket::{DataPush, PushAction, SubscriptionArgs};
    use chrono::{DateTime, Utc};

    #[test]
    pub fn test_deserialize_data_push() {
        let s = r#"
            {
                "action": "snapshot",
                "arg": {
                    "instType": "USDT-FUTURES",
                    "channel": "trade",
                    "instId": "BTCUSDT"
                },
                "data": [
                    {
                        "ts": "1695716760565",
                        "price": "27000.5",
                        "size": "0.001",
                        "side": "buy",
                        "tradeId": "1111111111"
                    },
                    {
                        "ts": "1695716759514",
                        "price": "27000.0",
                        "size": "0.001",
                        "side": "sell",
                        "tradeId": "1111111111"
                    }
                ],
                "ts": 1695716761589
            }
        "#;

        let expected = DataPush {
            action: PushAction::Snapshot,
            arg: SubscriptionArgs {
                inst_type: "USDT-FUTURES".to_owned(),
                channel: "trade".to_owned(),
                inst_id: "BTCUSDT".to_owned(),
            },
            data: vec![
                Trade {
                    ts: DateTime::from_timestamp_nanos(1695716760565),
                    price: 27000.5,
                    size: 0.001,
                    side: TradeSide::Buy,
                    trade_id: 1111111111,
                },
                Trade {
                    ts: DateTime::from_timestamp_nanos(1695716759514),
                    price: 27000.0,
                    size: 0.001,
                    side: TradeSide::Sell,
                    trade_id: 1111111111,
                },
            ],
            ts: DateTime::from_timestamp_nanos(1695716761589),
        };

        assert_eq!(
            serde_json::from_str::<DataPush<Trade>>(s).unwrap(),
            expected
        )
    }
}
