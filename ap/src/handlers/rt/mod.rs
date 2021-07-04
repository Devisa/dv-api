use std::collections::{HashMap, HashSet};
use std::sync::{Arc, atomic::AtomicUsize, };
use rand::Rng;
use rand::rngs::ThreadRng;
use std::{task::Poll, time::Duration};
use actix_web::{
    HttpRequest, http::{self, ContentEncoding},
};
use crate::actors::redis::RedisActor;
use crate::util::respond;
use futures::stream::poll_fn;
use actix::prelude::*;
use actix:: clock::Instant;
use actix_web::{Error, HttpResponse, Responder, http::StatusCode, web::{self, ServiceConfig, }};
use actix_web_actors::ws;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(index))
        .route("/sse", web::get().to(sse))
        .route("/ws", web::get().to(wsindex))
        .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .append_header(("LOCATION", "/static/websocket.html"))
                    .finish()
            })))
        .route("/count/", web::get().to(get_count))
        .service(web::resource("/ws/").to(chat_route))
        //.service(fs::Files::new("/static/", "static/"))
        .route("/events", web::get().to(events))
        .service(web::scope("/redis")
            .service(web::resource("/{key}")
                .route(web::get().to(get_redis_key))
                .route(web::post().to(add_redis_key_val))
                .route(web::delete().to(delete_redis_key))
            )
        )
        .route("/broadcast/{msg}", web::get().to(broad));
}
pub async fn index() -> impl Responder { "".to_owned() }
pub async fn events() -> impl Responder { "".to_owned() }
pub async fn broad() -> impl Responder { "".to_owned() }

async fn sse(req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 100;
    let server_events = poll_fn(move |_cx| -> Poll<Option<Result<web::Bytes, actix_web::Error>>> {
        if counter == 0 {
            return Poll::Ready(None);
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Poll::Ready(Some(Ok(web::Bytes::from(payload))))
    });
    HttpResponse::build(StatusCode::OK).streaming(server_events)
        .with_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .with_header((
            http::header::CONTENT_ENCODING,
            ContentEncoding::Identity.as_str(),
        ))
        .respond_to(&req)
}

async fn get_redis_key(redis: web::Data<RedisActor>, key: web::Path<String>) -> impl Responder {
    let key = key.into_inner();
    match redis.get(&key).await {
        Ok(r) => respond::ok(r),
        Err(e) => respond::err(e),
    }
}
async fn add_redis_key_val(redis: web::Data<RedisActor>, key: web::Path<String>, info: web::Json<String>) -> impl Responder {
    let (key, info) = (key.into_inner(), info.into_inner());
    match redis.set(&key, &info).await {
        Ok(r) => respond::ok(r),
        Err(e) => respond::err(e),
    }
}
async fn delete_redis_key(redis: web::Data<RedisActor>, key: web::Path<String>) -> impl Responder {
    let key = key.into_inner();
    match redis.del(&key).await {
        Ok(r) => respond::ok(r),
        Err(e) => respond::err(e)
    }
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn wsindex(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        self.send_message(&"Main".to_owned(), "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.send_message("Main", &format!("Total visitors {}", count), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms: Vec<String> = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

///  Displays and affects state
async fn get_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<ChatServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message to chat server and wait for
                            // response
                            println!("List rooms");
                            self.addr
                                .send(ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        }
                        "/join" => {
                            if v.len() == 2 {
                                self.room = v[1].to_owned();
                                self.addr.do_send(Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });

                                ctx.text("joined");
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{}: {}", name, m)
                    } else {
                        m.to_owned()
                    };
                    // send message to chat server
                    self.addr.do_send(ClientMessage {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.addr.do_send(Disconnect { id: act.id });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}


// pub async fn ws_index(
//     req: HttpRequest,
//     mut stream: web::Payload,
// ) -> Result<HttpResponse, Error> {
//     let mut res = handshake(&req)?;
//     let (tx, rx) = mpsc::unbounded_channel::<Result<Bytes, actix_web::Error>>();
//     tokio::task::spawn_local(async move {
//         while let Some(chunk) = stream.next().await {
//             let chunk = chunk.unwrap();
//             let mut codec = Codec::new();
//             let mut buf = BytesMut::new();
//             buf.extend_from_slice(&chunk[..]);

//             let frame = codec.decode(&mut buf).unwrap();
//             let frame_str = format!("frame: {:?}", frame);

//             let message = ws::Message::Text(frame_str);
//             let mut output_buffer = BytesMut::new();
//             codec.encode(message, &mut output_buffer).unwrap();
//             let b = output_buffer.split().freeze();
//             tx.send(Ok(b)).unwrap();
//         }
//     });
//
//
// pub async fn index() -> impl Responder {
//     let content = include_str!("../../../static/pages/rt.html");
//     let mut res = HttpResponseBuilder::new(StatusCode::OK)
//         .body(content)
//         .with_header(("content-type", "text/html"));
//     res
// }
// pub async fn new_client(broadcaster: web::Data<Mutex<Broadcaster>>) -> impl Responder {
//     let rx = broadcaster.lock().await.new_client();
//     let mut res = HttpResponseBuilder::new(StatusCode::OK)
//         .with_header(("content-type", "text/event-stream"));
//     res
// }
// pub async fn broadcast(msg: web::Path<String>, broadcaster: web::Data<Mutex<Broadcaster>>) -> impl Responder {
//     broadcaster.lock().await.send(&msg.into_inner());
//     HttpResponse::Ok().body("Message sent")
// }
// pub struct Broadcaster {
//     clients: Vec<Sender<Bytes>>
// }
// impl Broadcaster {
//     fn create() -> Data<Mutex<Self>> {
//         let m = Data::new(Mutex::new(Broadcaster::new()));
//         Broadcaster::spawn_ping(m.clone());
//         m
//     }
//     fn new() -> Self {
//         Broadcaster { clients: Vec::new() }
//     }
//     async fn spawn_ping(m: Data<Mutex<Self>>) {
//         let task = Interval::from(Instant::now().checked_add(Duration::from_secs(10)).unwrap().elapsed())
//             .for_each(move |_| async {
//                 m.lock().await.remove_stale_clients();
//                 Ok(())
//             }).map_err(|e| panic!("Interval errored; e = {:?}", e));
//         Arbiter::new().spawn(task);
//     }
//     fn remove_stale_clients(&mut self) {
//         let mut ok = vec![];
//         for c in self.clients.iter() {
//             let res = c.clone().try_send(Bytes::from("data: ping\n\n"));
//             if let Ok(()) = res {
//                 ok.push(c.clone());
//             }
//         }
//         self.clients = ok;
//     }
//     fn new_client(&mut self) -> Client {
//         let (tx, rx) = channel(100);
//         tx.clone().try_send(Bytes::from("data: connected\n\n"))
//             .unwrap();
//         self.clients.push(tx);
//         Client(rx)
//     }
//     fn send(&self, msg: &str) {
//         let m = Bytes::from(["data: ", msg, "\n\n"].concat());
//         for c in self.clients.iter() {
//             c.clone().try_send(m.clone()).unwrap_or(());
//         }
//     }
// }

// pub struct Client(Receiver<Bytes>);
// // impl futures::Stream for Client {
// //     type Item = Bytes;
// //     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
// //         &self.kk

// //     }
// // }
// impl actix::Actor for Client {
//     type Context = Context<Self>;
// }
