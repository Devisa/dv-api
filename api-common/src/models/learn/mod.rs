pub mod bot;
pub mod scrape;
pub mod hooks;
pub mod explore;
pub mod organize;
pub mod eval;
pub mod knowledge;

// use actix::prelude::*;
// use actix_broker::{Broker, BrokerSubscribe, SystemBroker};
// use actix_web::{
//     Responder, HttpRequest, HttpResponse,
// };
// use crate::{
//     db::Db,
//     util::respond,
//     types::{Status, now, private}
// };
// use serde::{Serialize, Deserialize};
// use sqlx::{
//     FromRow, Postgres,
//     types::chrono::{NaiveDateTime, Utc}
// };

// #[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
// pub struct LearningUnit {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub id: Option<i32>,
//     #[serde(default = "CheckbackRateDenominator::default_num")]
//     pub checkback_rate_num: i32,
//     #[serde(default = "CheckbackRateDenominator::default")]
//     pub checkback_rate_denom: CheckbackRateDenominator,
//     #[serde(default = "now")]
//     pub created_at: NaiveDateTime,
//     #[serde(default = "now")]
//     pub updated_at: NaiveDateTime,
// }

// #[derive(Debug, Clone, Message, Deserialize, Serialize)]
// #[rtype(result = "()")]
// pub struct LearningPayload {
//     data: usize,
//     text: Option<String>,
// }

// #[derive(Debug, Clone, Message, Deserialize, Serialize)]
// #[rtype(result = "()")]
// pub struct LearnedLink {

// }
// #[derive(Debug, Clone, Message, Deserialize, Serialize)]
// #[rtype(result = "()")]
// pub struct LearnedTopic {

// }

// #[derive(Debug,  Clone, Serialize, Deserialize)]
// pub struct LearningUnitNode {
//     pub id: usize,
//     pub limit: usize,
//     // pub node: Recipient<LearningPayload>
// }

// pub struct LearningUnitLinker {

// }

// pub struct LearningUnitExplorer {

// }

// impl LearningUnitNode {
// }

// impl Actor for LearningUnitNode {
//     type Context = Context<Self>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.subscribe_async::<SystemBroker, LearningPayload>(ctx);
//     }
// }

// impl Responder for LearningUnitNode {

//     fn respond_to(self, req: &HttpRequest) -> HttpResponse {
//         Broker::<SystemBroker>::issue_async(LearningPayload { data: 0, text: None });
//         respond::ok()
//             .body("Received")
//     }

// }

// impl Handler<LearningPayload> for LearningUnitNode {
//     type Result = ();

//     fn handle(&mut self, msg: LearningPayload, ctx: &mut Self::Context) -> Self::Result {
//         println!("Learning unit {} received learning payload {:#?}", self.id, msg);
//         if msg.data >= self.limit {
//             println!("Actor {} reached limit of {} ( payload was {} )", self.id, self.limit, msg.data);
//             System::current().stop();
//             return;
//         }
//         if msg.data % 498989 == 1 {
//             println!("Actor {} received message {} of {} ({:.2}%)",
//                 self.id,msg.data,self.limit,100.0*msg.data as f32/self.limit as f32);
//             // self.node.do_send(LearningPayload { data: msg.data + 1, text: None })
//             //     .expect("Unable to send payload");

//         }
//     }
// }

// #[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
// #[sqlx(rename = "checkback_rate_denominator", rename_all = "lowercase")]
// pub enum CheckbackRateDenominator {
//     Minute,
//     Hour,
//     Day,
//     Week,
//     Month,
// }

// impl CheckbackRateDenominator {
//     pub fn default_num() -> i32 {
//         1
//     }

//     pub fn default() -> Self {
//         Self::Day
//     }
// }

// #[cfg(test)]
// mod test {

//     use super::*;
//     use actix_rt;

//     #[actix_rt::test]
//     pub async fn test_round() -> std::io::Result<()> {
//         let sys = System::new();
//         let (n_nodes, n_rounds) = (20, 20);
//         let now = std::time::SystemTime::now();
//         sys.block_on(async {
//             println!("Setting up {} nodes", n_nodes);
//             let limit = n_nodes * n_rounds;
//             let node = LearningUnitNode::create(move |ctx| {
//                 let first_addr = ctx.address();
//                 let mut prev_addr = LearningUnitNode {
//                     id: 1,
//                     limit,
//                 }
//                 .start();
//                 for id in 2..n_nodes {
//                     prev_addr = LearningUnitNode {
//                         id,
//                         limit,
//                     }
//                     .start();
//                 }
//                 LearningUnitNode {
//                     id: n_nodes,
//                     limit,
//                 }
//             });
//             println!(
//                 "Sending start message and waiting for termination after {} messages...",
//                 limit
//             );
//             node.send(LearningPayload { data: 1, text: None }).await;
//         });
//         sys.run();
//         match now.elapsed() {
//             Ok(elapsed) => println!(
//                 "Time taken: {}.{:06} seconds ({} msg/second)",
//                 elapsed.as_secs(),
//                 elapsed.subsec_micros(),
//                 (n_nodes * n_rounds * 1000000) as u128 / elapsed.as_micros()
//             ),
//             Err(e) => println!("An error occurred: {:?}", e),
//         }

//         Ok(())
//     }
// }
