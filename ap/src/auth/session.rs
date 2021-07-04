
// use actix::prelude::{Actor, Addr, Context, Handler, Message};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use uuid::Uuid;

// #[derive(Debug, Clone)]
// pub struct WebSocketSession {
//     id: Uuid,
//     hb: Instant,
//     sessions: Addr<SessionService>,
//     pubsub: Addr<PubSubService>,
//     datalog: Addr<DataLogger>,
// }


// impl WebSocketSession {
//     fn new(
//         pubsub: &Addr<PubSubService>,
//         sessions: &Addr<SessionService>,
//         datalog: &Addr<DataLogger>,
//         client_id: &Uuid,
//     ) -> WebSocketSession {
//         WebSocketSession {
//             id: *client_id,
//             hb: Instant::now(),
//             sessions: sessions.clone(),
//             pubsub: pubsub.clone(),
//             datalog: datalog.clone(),
//         }
//     }

//     fn beat(&self, ctx: &mut <Self as Actor>::Context) {
//         ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
//             if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//                 warn!("Connection for {} timed out. Closing.", act.id);
//                 ctx.stop();
//                 return;
//             }
//             ctx.ping(b"");
//         });
//     }
// }

// impl Actor for WebSocketSession {
//     type Context = ws::WebsocketContext<Self>;

//     // On start of actor begin monitoring heartbeat and create
//     // a session on the `PubSubServer`
//     fn started(&mut self, ctx: &mut Self::Context) {
//         info!("Starting WebSocketSession for {}", self.id);
//         self.beat(ctx);
//         if let Err(e) = self
//             .sessions
//             .try_send(InsertSession::new(&self.id, &ctx.address()))
//         {
//             error!("{}", e);
//             ctx.stop()
//         }
//     }

//     // Unregister with SessionService when stopping the actor
//     fn stopping(&mut self, _: &mut Self::Context) -> Running {
//         info!("Stopping WebSocketSession for {}", self.id);
//         self.sessions.do_send(RemoveSession::from(&self.id));
//         Running::Stop
//     }
// }

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum SessionError {
//     SessionNotFound(String),
// }

// #[derive(Debug, Clone)]
// pub struct SessionInfo {
//     pub id: Uuid,
//     pub user_id: i32,
//     pub addr: Addr<WebSocketSession>,
// }

// #[derive(Debug, Message)]
// #[rtype("Result<SessionResponse, SessionError>")]
// pub enum SessionOp {
//     GetAddr { id: Uuid },
//     Insert(SessionInfo),
//     Remove { id: Uuid },
// }

// pub enum SessionResponse {
//     FoundAddress(Addr<WebSocketSession>),
//     DoNothing,

// }



// pub struct InsertSession {
//     id: Uuid,
//     addr: Addr<WebSocketSession>,
// }
// impl SessionOp {

//     pub fn insert(session: &SessionInfo) -> Self {
//         SessionOp::New(session)
//     }

//     pub fn remove(id: &Uuid) -> Self {
//         Self::Remove { id: id.clone() }
//     }
// }
// #[derive(Debug, PartialEq, Clone)]
// pub struct SessionService {
//     sessions: Vec<SessionInfo>
// }

// impl Actor for SessionService {
//     type Context = Context<Self>;
// }

// impl Handler<SessionOp> for SessionService {
//     type Result = Result<SessionResponse, SessionError>;

//     fn handle(&mut self, msg: SessionOp, _: &mut Context<Self>) -> Self::Result {
//         match msg {
//             SessionOp::Insert(session) => {
//                 self.insert_session(&session);
//                 Ok(SessionResponse::DoNothing)
//             },
//             SessionOp::Remove { id } => {
//                 self.remove_session(&id);
//                 Ok(SessionResponse::DoNothing)
//             },
//             SessionOp::GetAddr { id } => {
//                 let res = self.get_session_addr(&id)?;
//                 Ok(SessionResponse::FoundAddress(res))
//             },
//         }
//     }
// }

// impl SessionService {
//     pub fn new() -> Self {
//         SessionService {
//             sessions: Vec<SessionInfo>
//         }
//     }

//     fn insert_session(&mut self, session: &SessionInfo) {
//         self.sessions.insert(client_id.clone(), addr.clone());
//     }

//     fn remove_session(&mut self, client_id: &Uuid) {
//         self.sessions.remove(client_id);
//     }

//     fn get_session_addr(&self, client_id: &Uuid) -> Result<&Addr<WebSocketSession>, SessionError> {
//         if let Some(entry) = self.sessions.get(client_id) {
//             Ok(entry)
//         } else {
//             Err(SessionError::SessionNotFound(client_id.to_string()))
//         }
//     }
// }
