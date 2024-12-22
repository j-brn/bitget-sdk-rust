use crate::de::{datetime_from_timestamp_str, parse_from_str};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Eq, PartialEq)]
pub enum SymbolType {
    Perpetual = 1,
    Delivery = 2,
}

impl<'de> Deserialize<'de> for SymbolType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            1 => Ok(SymbolType::Perpetual),
            2 => Ok(SymbolType::Delivery),
            x => Err(serde::de::Error::custom(format!("Invalid symbolType: {x}"))),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    pub inst_id: String,
    #[serde(deserialize_with = "parse_from_str")]
    #[serde(rename = "lastPr")]
    pub last_price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    #[serde(rename = "bidPr")]
    pub bid_price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    #[serde(rename = "askPr")]
    pub ask_price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    #[serde(rename = "bidSz")]
    pub bid_size: f64,
    #[serde(deserialize_with = "parse_from_str")]
    #[serde(rename = "askSz")]
    pub ask_size: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub open_24h: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub high_24h: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub low_24h: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub change_24h: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub funding_rate: f64,
    #[serde(deserialize_with = "datetime_from_timestamp_str")]
    pub next_funding_time: DateTime<Utc>,
    #[serde(deserialize_with = "parse_from_str")]
    pub mark_price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub index_price: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub holding_amount: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub base_volume: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub quote_volume: f64,
    #[serde(deserialize_with = "parse_from_str")]
    pub open_utc: f64,
    pub symbol_type: SymbolType,
    pub symbol: String,
    #[serde(deserialize_with = "parse_from_str")]
    pub delivery_price: f64,
    #[serde(deserialize_with = "datetime_from_timestamp_str")]
    #[serde(rename = "ts")]
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use crate::future::public::ticker::{SymbolType, Tick};
    use crate::websocket::{DataPush, PushAction, SubscriptionArgs};
    use chrono::DateTime;

    #[test]
    pub fn test_deserialize_ws_push() {
        let s = r#"
            {
              "action": "snapshot",
              "arg": {
                "instType": "USDT-FUTURES",
                "channel": "ticker",
                "instId": "BTCUSDT"
              },
              "data": [
                {
                  "instId": "BTCUSDT",
                  "lastPr": "27000.5",
                  "bidPr": "27000",
                  "askPr": "27000.5",
                  "bidSz": "2.71",
                  "askSz": "8.76",
                  "open24h": "27000.5",
                  "high24h": "30668.5",
                  "low24h": "26999.0",
                  "change24h": "-0.00002",
                  "fundingRate": "0.000010",
                  "nextFundingTime": "1695722400000",
                  "markPrice": "27000.0",
                  "indexPrice": "25702.4",
                  "holdingAmount": "929.502",
                  "baseVolume": "368.900",
                  "quoteVolume": "10152429.961",
                  "openUtc": "27000.5",
                  "symbolType": 1,
                  "symbol": "BTCUSDT",
                  "deliveryPrice": "0",
                  "ts": "1695715383021"
                }
              ],
              "ts": 1695715383039
            }
        "#;

        let expected = DataPush {
            action: PushAction::Snapshot,
            arg: SubscriptionArgs {
                inst_type: "USDT-FUTURES".to_owned(),
                channel: "ticker".to_owned(),
                inst_id: "BTCUSDT".to_owned(),
            },
            data: vec![Tick {
                inst_id: "BTCUSDT".to_owned(),
                last_price: 27000.5,
                bid_price: 27000.0,
                ask_price: 27000.5,
                bid_size: 2.71,
                ask_size: 8.76,
                open_24h: 27000.5,
                high_24h: 30668.5,
                low_24h: 26999.0,
                change_24h: -0.00002,
                funding_rate: 0.000010,
                next_funding_time: DateTime::from_timestamp_nanos(1695722400000),
                mark_price: 27000.0,
                index_price: 25702.4,
                holding_amount: 929.502,
                base_volume: 368.900,
                quote_volume: 10152429.961,
                open_utc: 27000.5,
                symbol_type: SymbolType::Perpetual,
                symbol: "BTCUSDT".to_owned(),
                delivery_price: 0.0,
                timestamp: DateTime::from_timestamp_nanos(1695715383021),
            }],
            ts: DateTime::from_timestamp_nanos(1695715383039),
        };

        assert_eq!(serde_json::from_str::<DataPush<Tick>>(s).unwrap(), expected)
    }
}
