use actix::prelude::*;
use actix_web::web::{get, Path, ServiceConfig};
use uuid::Uuid;
use crate::{util::respond, Id,  Status, now, private};
use crate::rel::link::{LinkedTo, Linked};
use crate::models::{Model, Link};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc, }
};
use super::{Field,User, Topic, item::Item};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordRelation {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Id::nil")]
    pub record1_id: Id,
    #[serde(default = "Id::nil")]
    pub record2_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordItem {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Id::nil")]
    pub record_id: Id,
    #[serde(default = "Id::nil")]
    pub item_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for RecordItem {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            record_id: Id::nil(),
            item_id: Id::nil(),
            link_id: None,
            name: None,
            description: None,
            status: Status::default(),
            created_at: now(),
            updated_at: now(),
        }
    }
}

#[async_trait::async_trait]
impl LinkedTo<Item> for Record {
    type LinkModel = RecordItem;
    #[inline]
    fn path() -> String { String::from("/{record_id}/item") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|record_id: Path<Id>| respond::ok(format!("GET /record/{}/item/hi", &record_id))));
    }
}

#[async_trait::async_trait]
impl Linked for RecordItem {
    type Left = Record;
    type Right = Item;

    fn new_basic(left_id: Id, right_id: Id, link_id: Option<Id>) -> Self {
        Self {
            id: Id::gen(),
            record_id: left_id, item_id: right_id, link_id,
            ..Default::default()
        }
    }

    fn link_id(self) -> Option<Id> { self.link_id }
    fn left_id(self) -> Id { self.record_id }
    fn right_id(self) -> Id { self.item_id }

    /// Served at /record/{record_id}/item/{item_id}
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<(Id, Id)>| {
                let (record_id, item_id) = id.into_inner();
                respond::ok(format!("GET /record/{}/item/{}/hi", &record_id, &item_id))
            }
            )
        );
    }
}

impl RecordItem {
    pub fn new(record_id: Id, item_id: Id, link_id: Option<Id>, name: Option<String>, description: Option<String>) -> Self {
        Self {
            record_id, item_id, link_id, name, description, ..Default::default()
        }

    }
}

#[async_trait::async_trait]
impl Model for Record {
    #[inline]
    fn table() -> String { String::from("records") }
    #[inline]
    fn path() -> String { String::from("/record") }
    fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /record/hi".to_string())))
            .service(<Record as LinkedTo<Item>>::scope())
            .service(<RecordItem as Linked>::scope());
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_scalar("
            INSERT INTO records (name, user_id, private, status, description, image, cover_image, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id
            ")
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.private)
            .bind(&self.status)
            .bind(&self.description)
            .bind(&self.image)
            .bind(&self.cover_image)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(Record { id: res, ..self })
    }
}
#[async_trait::async_trait]
impl Model for RecordItem {
    #[inline]
    fn table() -> String { String::from("record_items") }

    #[inline]
    fn path() -> String { String::from("/item") }

    fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg;
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO record_items
            (record_id, item_id, link_id, name, description, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ")
            .bind(self.record_id)
            .bind(self.item_id)
            .bind(self.link_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl Record {

    pub fn new(name: String, user_id: Id) -> Self {
        Self {
            user_id, name, ..Default::default()
        }
    }

    pub async fn update_by_id(db: &PgPool, id: Id, r: Record)
        -> anyhow::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE records
            SET        name = $1
                       description = $2
                       private = $3
                       image = $4
                       status = $5
                       cover_image = $6
                       updated_at = $7
            WHERE      id = $8
            RETURNING  id
            ")
            .bind(&r.name)
            .bind(&r.description)
            .bind(&r.private)
            .bind(&r.image)
            .bind(&r.status)
            .bind(&r.cover_image)
            .bind(now())
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_all_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Record>("SELECT * FROM records WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn add_new_item(self, db: &PgPool, item_name: String) -> anyhow::Result<Self> {
        let item = Item::new(item_name, self.clone().user_id).insert(&db).await?;
        let item_link = RecordItem::new(self.clone().id, item.id, None, None, None).insert(&db).await?;
        Ok(Self { ..self })
    }

    pub async fn add_existing_item(self, db: &PgPool, item_id: Id) -> anyhow::Result<RecordItem> {
        let item = Item::get(&db, item_id).await?;
        if let Some(item) = item {
            let item_link = RecordItem::new(self.clone().id, item.id, None, None, None).insert(&db).await?;
            Ok(item_link)
        } else {
            return Err(anyhow::anyhow!("Item does not exist"));
        }
    }

    // pub async fn interpret_record_reaction(self, reaction: RecordReaction) {
    //     match reaction {
    //         RecordReaction::TrackRecord(rec) => {
    //             if self == rec {
    //                 println!("This record was noticed by other records due to this post");
    //             } else {
    //                 println!("This record caused other records to begin tracking records");
    //             }
    //         },
    //         RecordReaction::TrackUser(user) => println!("RECEIVED: Records began tracking {}", user.email),
    //         RecordReaction::TrackTopic(topic) => println!("RECEIVED: Records began tracking {}", topic.name),
    //         _ => println!("RECEIVD: Other record reaction")
    //     }

    // }

    // pub async fn add_post(self, post: RecordPost) -> anyhow::Result<()> {
    //     let record = self.start();
    //     let new_post = record.send(RecordEvent::newPost(post)).await;
    //     let new_user = record.send(RecordEvent::NewUserVisit(post.user)).await;
    //     self.interpret_record_reaction(new_user);
    //     self.interpret_record_reaction(new_post);

    // }
}

#[derive(Serialize, Deserialize)]
pub struct RecordName {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRecord {
    pub name: String,
    pub user_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRecordItem {
    pub name: String,
    pub item: String,
    pub user_id: Id,
}

impl Default for Record {
    fn default() -> Self {
        Record {
            id: Id::gen(),
            user_id: Id::nil(),
            name: String::new(),
            private: true,
            status: Status::Active,
            description: None,
            image: None,
            cover_image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Actor for Record {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Record learning unit has initiated");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Record learning unit has terminated");

    }

}

#[derive(Message)]
#[rtype(result = "RecordReaction")]
pub enum RecordEvent {
    NewItem(Item),
    ItemUpdate(Item, Item),
    ItemDropped(Item),
    NewFieldValue(String),
    NewPost(RecordPost),
    NewMention(String),
    NewUserVisit(User),
    NewTopic(String),
    NewLink(String),
    UserAdded(User),
    UserLeft(User),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPost {
    id: Id,
    user: User,
    content: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordReaction {
    TrackTopic(Topic),
    TrackRecord(Record),
    DistanceTopic(Topic),
    TrackItem(Item),
    TrackLinked(Link),
    DistanceItem(Item),
    TrackUser(User),
    TrackField(Field),
    ParseText(String),
}

// impl<A, M> MessageResponse<A, M> for RecordReaction
// where
//     A: Actor,
//     M: Message<Result = RecordReaction>
// {
//     fn handle<R: RecordEvoChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
//         if let Some(tx) = tx {
//             tx.send(self)
//         }
//     }
// }


// impl Handler<RecordEvent> for Record {
//     type Result = RecordReaction;

//     fn handle(&mut self, msg: RecordEvent, _ctx: &mut Context<Self>) -> Self::Result {
//         println!("Record {} received notice that a record has had an event.", self.name);
//         match msg {
//             RecordEvent::NewItem(item) => RecordReaction::TrackItem(item),
//             RecordEvent::ItemDropped(item) => RecordReaction::DistanceItem(item),
//             RecordEvent::ItemUpdate(item) => RecordReaction::TrackItem(item),
//             RecordEvent::NewFieldValue(field) => RecordReaction::TrackField(field),
//             RecordEvent::NewLink(link) => RecordReaction::TrackLinked(link),
//             RecordEvent::NewTopic(topic) => RecordReaction::TrackTopic(topic),
//             RecordEvent::UserAdded(user) => RecordReaction::TrackUser(user),
//             RecordEvent::NewUserVisit(user) => RecordReaction::TrackUser(user),
//             RecordEvent::UserLeft(user) => RecordReaction::TrackUser(user),
//             RecordEvent::NewPost(post) => RecordReaction::ParseText(post),
//             RecordEvent::NewMention(post) => RecordReaction::ParseText(post),
//         }

//     }

// }
