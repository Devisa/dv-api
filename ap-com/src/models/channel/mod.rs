use actix::prelude::*;
use crate::{Id, Model};
use crate::now;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Channel {

    fn table() -> String { String::from("channels") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO channels (id, user_id, name, description)
             vALUES ($1, $2, $3, $4) RETURNING *")
            .bind(&self.id)
            .bind(self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .fetch_one(db).await?;
        Ok(res)
    }
}

pub struct TopicChannel {}
pub struct GroupChannel {}
pub struct RecordChannel {}

impl Channel {

    pub fn new(user_id: Id, name: String, description: Option<String>) -> Self {
        Self {
            id: Id::gen(),
            updated_at: now(),
            created_at: now(),
            user_id, name, description
        }
    }
}

impl Actor for Channel {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("CHANNEL ACTOR STARTED: {:?}", self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("CHANNEL ACTOR STOPPED: {:?}", self.id);
    }
}

// impl Default for ChannelServer {
//     fn default() -> ChannelServer {
//         let mut rooms = HashMap::new();
//         rooms.insert("Main".to_owned(), HashSet::new());
//         ChannelServer {
//             sessions: HashMap::new(),
//             rooms,
//             rng: rand::thread_rng(),
//         }
//     }
// }

// impl ChannelServer {
//     fn send_message(&self, room: &str, message: &str, skip_id: usize) {
//         if let Some(sessions) = self.rooms.get(room) {
//             for id in sessions {
//                 if *id != skip_id {
//                     if let Some(addr) = self.sessions.get(id) {
//                         let _ = addr.do_send(session::Message(message.to_owned()));
//                     }
//                 }
//             }
//         }
//     }
// }

// impl Actor for ChannelServer {
//     type Context = Context<Self>;
// }

// impl Handler<SessionConnect> for ChatServer {
//     type Result = usize;

//     fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
//         println!("Someone joined");
//         self.send_message(&"Main".to_owned(), "Someone joined", 0);
//         let id = self.rng.gen::<usize>();
//         self.sessions.insert(id, msg.addr);
//         self.rooms.get_mut(&"Main".to_owned()).unwrap().insert(id);
//         id
//     }
// }
// impl Handler<SessionDisconnect> for ChatServer {
//     type Result = ();

//     fn handle(&mut self, msg: SessionDisconnect, _: &mut Context<Self>) {
//         println!("Someone disconnected");
//         let mut rooms: Vec<String> = Vec::new();
//         if self.sessions.remove(&msg.id).is_some() {
//             for (name, sessions) in &mut self.rooms {
//                 if sessions.remove(&msg.id) {
//                     rooms.push(name.to_owned());
//                 }
//             }
//         }
//         for room in rooms {
//             self.send_message(&room, "Someone disconnected", 0);
//         }
//     }
// }
// impl Handler<Message> for ChannelServer {
//     type Result = ();

//     fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
//         self.send_message(&msg.room, msg.msg.as_str(), msg.id);
//     }
// }
// impl Handler<ListRooms> for ChannelServer {
//     type Result = MessageResult<ListRooms>;

//     fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
//         let mut rooms = Vec::new();
//         for key in self.rooms.keys() {
//             rooms.push(key.to_owned())
//         }
//         MessageResult(rooms)
//     }
// }
// impl Handler<Join> for ChatServer {
//     type Result = ();

//     fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
//         let Join { id, name } = msg;
//         let mut rooms = Vec::new();
//         for (n, sessions) in &mut self.rooms {
//             if sessions.remove(&id) {
//                 rooms.push(n.to_owned());
//             }
//         }
//         for room in rooms {
//             self.send_message(&room, "Someone disconnected", 0);
//         }
//         if self.rooms.get_mut(&name).is_none() {
//             self.rooms.insert(name.clone(), HashSet::new());
//         }
//         self.send_message(&name, "Someone connected", id);
//         self.rooms.get_mut(&name).unwrap().insert(id);
//     }
// }



// pub struct ChannelSession {
//     pub id: Option<Id>,
//     pub addr: Addr<ChannelServer>,
//     pub hb: Instant,
//     pub room: String,
//     pub framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
// }

// impl ChannelSession {
//     pub fn new(
//         addr: Addr<ChatServer>,
//         framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
//     ) -> ChatSession {
//         ChatSession {
//             id: 0,
//             addr,
//             hb: Instant::now(),
//             room: "Main".to_owned(),
//             framed,
//         }
//     }

//     fn hb(&self, ctx: &mut Context<Self>) {
//         ctx.run_interval(Duration::new(1, 0), |act, ctx| {
//             if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
//                 println!("Client heartbeat failed, disconnecting!");
//                 act.addr.do_send(server::Disconnect { id: act.id });
//                 ctx.stop();
//             }
//             act.framed.write(ChatResponse::Ping);
//         });
//     }
// }

// impl Actor for ChannelSession {
//     type Context = Context<Self>;

//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.hb(ctx);
//         let addr = ctx.address();
//         self.addr.send(SessionConnect {
//             addr: addr.recipient(),
//         })
//         .into_actor(self)
//             .then(|res, act, ctx| {
//                 match res {
//                     Ok(res) => act.id = res,
//                     _ => ctx.stop(),
//                 }
//                 actix::fut::ready(())
//             })
//         .wait(ctx)
//     }

//     fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
//         self.addr.do_send(SessionDisconnect { id: self.id });
//         Running::Stop
//     }
// }

// impl actix::io::WriteHandler<io::Error> for ChatSession {}

// impl StreamHandler<Result<ChatRequest, io::Error>> for ChatSession {
//     fn handle(&mut self, msg: Result<ChatRequest, io::Error>, ctx: &mut Context<Self>) {
//         match msg {
//             Ok(ChatRequest::List) => {
//                 println!("List rooms");
//                 self.addr
//                     .send(server::ListRooms)
//                     .into_actor(self)
//                     .then(|res, act, _| {
//                         match res {
//                             Ok(rooms) => act.framed
//                                 .write(ChatResponse::Rooms(rooms)),
//                             _ => println!("Something is wrong"),
//                         }
//                         actix::fut::ready(())
//                     })
//                     .wait(ctx)
//             },
//             Ok(ChatRequest::Join(name)) => {
//                 println!("Join to room: {}", name);
//                 self.room = name.clone();
//                 self.addr.do_send(server::Join {
//                     id: self.id,
//                     name: name.clone(),
//                 });
//                 self.framed.write(ChatResponse::Joined(name));
//             },
//             Ok(ChatRequest::Message(message)) => {
//                 println!("Peer message: {}", message);
//                 self.addr.do_send( {
//                     id: self.id,
//                     msg: message,
//                     room: self.room.clone(),
//                 })
//             },
//             Ok(ChatRequest::Ping) => self.hb = Instant::now(),
//             _ => ctx.stop(),
//         }
//     }
// }

// impl Handler<ChannelMessage> for ChatSession {
//     type Result = ();

//     fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
//         self.framed.write(ChatResponse::Message(msg.0));
//     }
// }

// pub struct TopicChannel {
//     pub id: Option<Id>,
//     pub channel_id: Id,
//     pub topic_id: Id,
//     pub rooom: String,
//     pub private: bool,
// }

// pub struct RecordChannel {
//     pub id: Option<Id>,
//     pub channel_id: Id,
//     pub record_id: Id,
//     pub rooom: String,
//     pub private: bool,
// }

// pub struct GroupChannel {
//     pub id: Option<Id>,
//     pub channel_id: Id,
//     pub group_id: Id,
//     pub rooom: String,
//     pub private: bool,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct ChannelMessage {
//     pub id: Option<Id>,
//     pub user_id: Id,
//     pub channel_id: Id,
//     pub sent: NaiveDateTime,
// }

// pub enum ChannelOp {
//     Connect(Recipient<),
//     Disconnect,
// }

// pub struct SessionConnect {
//     pub addr: Recipient<
// }


// pub struct ChannelUserSession {

// }

// pub fn tcp_server(_s: &str, server: Addr<ChatServer>) {
//     // Create server listener
//     let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();

//     actix_web::rt::spawn(async move {
//         let server = server.clone();
//         let mut listener = TcpListener::bind(&addr).await.unwrap();
//         let mut incoming = listener.incoming();

//         while let Some(stream) = incoming.next().await {
//             match stream {
//                 Ok(stream) => {
//                     let server = server.clone();
//                     ChatSession::create(|ctx| {
//                         let (r, w) = split(stream);
//                         ChatSession::add_stream(FramedRead::new(r, ChatCodec), ctx);
//                         ChatSession::new(
//                             server,
//                             actix::io::FramedWrite::new(w, ChatCodec, ctx),
//                         )
//                     });
//                 }
//                 Err(_) => return,
//             }
//         }
//     });
// }

