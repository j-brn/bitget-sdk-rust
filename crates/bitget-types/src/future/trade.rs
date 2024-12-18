use crate::de::{datetime_from_timestamp_str, parse_f64, parse_usize};
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
pub struct TradeData {
    #[serde(deserialize_with = "datetime_from_timestamp_str")]
    pub ts: DateTime<Utc>,
    #[serde(deserialize_with = "parse_f64")]
    pub price: f64,
    #[serde(deserialize_with = "parse_f64")]
    pub size: f64,
    pub side: TradeSide,
    #[serde(deserialize_with = "parse_usize")]
    pub trade_id: usize,
}

#[cfg(test)]
mod tests {
    use crate::future::trade::{TradeData, TradeSide};
    use crate::websocket::{DataPush, SubscriptionArgs};
    use chrono::{DateTime, Utc};

    #[test]
    pub fn test_deserialize() {
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
            action: "snapshot".to_owned(),
            arg: SubscriptionArgs {
                inst_type: "USDT-FUTURES".to_owned(),
                channel: "trade".to_owned(),
                inst_id: "BTCUSDT".to_owned(),
            },
            data: vec![
                TradeData {
                    ts: DateTime::from_timestamp_nanos(1695716760565),
                    price: 27000.5,
                    size: 0.001,
                    side: TradeSide::Buy,
                    trade_id: 1111111111,
                },
                TradeData {
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
            serde_json::from_str::<DataPush<TradeData>>(s).unwrap(),
            expected
        )
    }
}
