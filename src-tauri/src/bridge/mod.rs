pub mod events;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DataRow {
    title: &'static str,
    row_title: String,
    timestamp: String,
    packet_id: String,
    raw_data: String,
}
