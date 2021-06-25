use actix::prelude::*;
use crate::types::*;

pub struct ApiKeys {
    token: String,
    master: String,
}

#[derive(Default)]
pub struct Scraper;

impl Actor for Scraper {
    type Context = Context<Self>;
}

impl Supervised for Scraper {}

impl SystemService for Scraper {}

// impl Handler<Msg> for Scraper {
//     type Result = ();

//     fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) {
//         let bot = TelegramBot::from_registry();

//         let send_text = |text| {
//             bot.do_send(RunResult {
//                 from_user: match msg {
//                     Msg::User => true,
//                     _ => false,
//                 },
//                 text,
//             })
//         };

//         let scraper = std::process::Command::new("./scraper").output();

//         match scraper {
//             Err(e) => send_text(format!("Error running scraper: {}", e)),
//             Ok(output) => {
//             // ...
//                     send_text(stdout.to_string());
//             // ...
//         }
//     }
// }
// }
