//
use actix::prelude::*;
// use crate::models::{Post, Record};
use serde::{Serialize, Deserialize};
// use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct Bot {
    id: Option<i32>,
    user_id: i32,
    record_id: i32,
}
    // memory: TopicMemory,

// #[derive(Serialize, Deserialize)]
// pub struct TopicMemory {
//     topic_nodes: HashSet<(i32,), String>,
//     topic_edges: HashMap<(i32, i32), String>,
//     item_topics: HashMap<(i32, i32), String>,
//     record_topics: HashMap<(i32, i32), String>,
// }

// impl Bot {

//     pub fn new(record: Record) -> Self {
//         S

//     }
// }


// #[derive(Message, Serialize, Serialize)]
// #[rtype(result = "()")]
// pub enum MessageSent {
//     Command(BotCommand),
//     ChatMessage(Message),
//     NewPost(Post),
//     ExternaChatTrigger(String),
//     ExternaCommandTrigger(String)
// }

// pub enum BotCommand {
//     SubscribeTopic(i32),
//     SubscribeRecord(i32),
//     AddTopicNote(i32, String),
//     AddTopicRelation((i32, i32), String),
//     AddItemTopicLink((i32, i32), String),
//     AddRecordTopicLink((i32, i32), String),
// }


// // Provide Actor implementation for our actor
// impl Actor for Bot {

//     type Context = Context<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         ctx.set_mailbox_capacity(32);
//         println!("Bot started");
//     }

//     fn stopped(&mut self, ctx: &mut Self::Context) {
//         println!("Bot stopped");
//     }


// }
// // Define handler for `Ping` message
// impl Handler<Ping> for Bot {
//     type Result = Result<bool, io::Error>;

//     fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>)
//          -> Self::Result {
//         println!("Ping received");
//         Ok(true)
//     }
// }
