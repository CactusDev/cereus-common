
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum Component {
    Text(String),
    Emoji(String),
    Tag(String),
    URL(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    Start { new: bool },
    Follow { success: bool },
    Subscribe { streak: usize },
    Host { success: bool },
    Join { success: bool },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Packet {
    Message { text: Vec<Component>, action: bool },
    Ban { duration: Option<usize> },
    Event { kind: Event },
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Banned,
    User,
    Subscriber,
    Moderator,
    Owner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub packet: Vec<Packet>,
    pub channel: String,
    pub user: Option<String>,
    pub role: Option<Role>,
    pub target: Option<String>,
    pub service: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginPacket {
    pub context: Context,
    pub target: String
}

pub trait ToCactusPacket {
    fn to_cactus_packet(&mut self) -> Packet;
}
