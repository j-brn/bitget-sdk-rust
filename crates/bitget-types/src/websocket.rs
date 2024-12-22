use crate::de::datetime_from_timestamp;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionArgs {
    pub(crate) inst_type: String,
    pub(crate) channel: String,
    pub(crate) inst_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PushAction {
    Snapshot,
    Update,
}

/// Container format for server side data messages
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct DataPush<T> {
    pub(crate) action: PushAction,
    pub(crate) arg: SubscriptionArgs,
    pub(crate) data: Vec<T>,
    #[serde(deserialize_with = "datetime_from_timestamp")]
    pub(crate) ts: DateTime<Utc>,
}
