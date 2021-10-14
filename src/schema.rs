use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "created_by")]
    pub created_by: Option<serde_json::Value>,

    #[serde(rename = "public_id")]
    pub public_id: String,

    #[serde(rename = "updated_on")]
    pub updated_on: String,

    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "updated_by")]
    pub updated_by: Option<serde_json::Value>,

    #[serde(rename = "created_user")]
    pub created_user: Option<serde_json::Value>,

    #[serde(rename = "is_editable")]
    pub is_editable: bool,

    #[serde(rename = "value")]
    pub value: Option<Vec<Vec<Value>>>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "old_id")]
    pub old_id: Option<serde_json::Value>,

    #[serde(rename = "created_on")]
    pub created_on: String,

    #[serde(rename = "updated_user")]
    pub updated_user: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Double(f64),

    String(String),
}
