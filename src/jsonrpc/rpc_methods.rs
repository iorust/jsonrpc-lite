use super::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Request {
    pub(super) jsonrpc: String,
    pub(super) method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) params: Option<Params>,
    pub(super) id: Id,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub(super) jsonrpc: String,
    pub(super) method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) params: Option<Params>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Success {
    pub(super) jsonrpc: String,
    pub(super) result: Value,
    pub(super) id: Id,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Error {
    pub(super) jsonrpc: String,
    pub(super) error: RpcError,
    pub(super) id: Id,
}
