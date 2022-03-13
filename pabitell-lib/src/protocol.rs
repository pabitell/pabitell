use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "msg", rename_all = "snake_case")]
pub enum Message {
    Notification(NotificationMessage),
    Request(RequestMessage),
    Reponse(ResponseMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "notification", rename_all = "snake_case")]
pub enum NotificationMessage {
    /// Triggered event
    Event(EventNotification),
    /// Character joined
    Joined(JoinedNotification),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinedNotification {
    pub character: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventNotification {
    pub event: Value,
    pub event_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum RequestMessage {
    /// New world created
    CreateNewWorld(Uuid),
    /// New world obtained
    GetWorld(GetWorldRequest),
    TriggerEvent(TriggerEventMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWorldRequest {
    pub msg_id: Uuid,
    pub world_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerEventMessage {
    pub msg_id: Uuid,
    pub event: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum ResponseMessage {
    CreateNewWorld(CreateWorldResponse),
    GetWorld(GetWorldResponse),
    TriggerEvent(Uuid),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWorldResponse {
    pub msg_id: Uuid,
    pub world: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWorldResponse {
    pub msg_id: Uuid,
    pub new_world: Uuid,
}
