use uuid::Uuid;
use actix_web::web::{self, Path, get, ServiceConfig};
use actix::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres,
    prelude::*, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};
use crate::{Linked, LinkedTo, models::{
        Model,
        category::Category,
        book::topic::TopicBook,
        post::Post,
    }, types::{Id, Status, now, private, Feeling}, util::respond};

#[derive(Debug, Clone, Serialize, Deserialize, )]
pub struct ScoreRequest {
    #[serde(default = "Id::gen")]
    pub user_id: Id,
    #[serde(default = "ScoreRequest::zero")]
    pub score: f64,
}

impl Default for ScoreRequest {
    #[inline]
    fn default() -> Self {
        Self {
            user_id: Id::gen(),
            score: 0.0
        }
    }
}

impl ScoreRequest {
    #[inline]
    fn zero() -> f64 { 0.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, )]
pub enum TopicLinkType {
    Topic(TopicLink),
    Group(TopicGroupSubscriber),
    User(TopicUserSubscriber),
    Post(TopicPost),
    Book(TopicBook),
    Link(TopicLink),
    Record(TopicLink),
    Item(TopicLink),
    Field(TopicLink),
    Action(TopicLink),
    Condition(TopicLink),
    Automation(TopicLink),
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    #[serde(default = "Id::gen")]
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicPost {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub post_id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
/// LINK MODEL: Lists topic's categories, vice versa
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicCategory {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::gen")]
    pub user_id: Id,
    #[serde(default = "Id::gen")]
    pub category_id: Id,
    #[serde(default = "Id::gen")]
    pub topic_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_id: Option<Id>,
    pub score: f64,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicVote {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    pub is_for: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeling: Option<Feeling>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

#[async_trait::async_trait]
impl Model for Topic {
    #[inline]
    fn path() -> String { String::from("/topic") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /topic/hi".to_string())))
            .service(<TopicVote as Model>::scope())
            .service(<Topic as LinkedTo<Category>>::scope())
            .service(<TopicCategory as Linked>::scope())
            .service(<TopicPost as Linked>::scope());

    }
    #[inline]
    fn table() -> String { String::from("topics") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let top = sqlx::query_as::<Postgres, Topic>("
            INSERT INTO topics (id, name, description)
            VALUES ($1, $2, $3)
            RETURNING *")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.description)
            .fetch_one(db).await?;
        Ok(top)
    }
}

impl Default for TopicCategory {
    fn default() -> Self {
        Self {
            link_id: None,
            id: Id::gen(),
            user_id: Id::nil(),
            category_id: Id::nil(),
            topic_id: Id::nil(),
            score: 0.0,
            updated_at: now(),
            created_at: now(),
        }
    }
}
#[async_trait::async_trait]
impl Model for TopicCategory {
    #[inline]
    fn path() -> String { String::from("/category") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /topic/category/hi".to_string())));
    }
    #[inline]
    fn table() -> String { String::from("topic_categories") }
    #[inline]
    fn id_str() -> String { String::from("topic_category_id") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res =  sqlx::query_as::<Postgres, TopicCategory>("
            INSERT INTO topic_categories (id, user_id, category_id,  topic_id, score)
            VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(self.id)
            .bind(self.user_id)
            .bind(self.category_id)
            .bind(self.topic_id)
            .bind(self.score)
            .fetch_one(db).await?;
        Ok(res)
    }
}
#[async_trait::async_trait]
impl Model for TopicVote {
    #[inline]
    fn table() -> String { String::from("topic_votes") }

    #[inline]
    fn path() -> String { String::from("/vote") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /topic/vote/hi".to_string())));
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res =  sqlx::query_as::<Postgres, TopicVote>("
            INSERT INTO topic_votes (id, user_id,  topic_id, is_for, description, feeling)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(self.id)
            .bind(self.user_id)
            .bind(self.topic_id)
            .bind(self.is_for)
            .bind(&self.description)
            .bind(&self.feeling)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl LinkedTo<Category> for Topic {
    type LinkModel = TopicCategory;

    fn path() -> String { String::from("/{category_id}/category") }

    /// Served at /topic/{topic_id}/category
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<Id>| respond::ok(format!("GET /topic/{}/category/hi", &id))));
    }
}
#[async_trait::async_trait]
impl LinkedTo<Post> for Topic {
    type LinkModel = TopicPost;

    fn path() -> String { String::from("/{topic_id}/post") }

    /// Served at /topic/{topic_id}/post
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<Id>| respond::ok(format!("GET /topic/{}/post/hi", &id))));
    }
}

impl Default for TopicPost {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            post_id: Id::nil(),
            topic_id: Id::nil(),
            link_id: None,
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl TopicPost {

    pub fn new(post_id: Id, topic_id: Id, link_id: Option<Id>) -> Self {
        Self {
            id: Id::gen(),
            post_id: Id::nil(),
            topic_id: Id::nil(),
            link_id: None,
            created_at: now(),
            updated_at: now(),
        }
    }
}
#[async_trait::async_trait]
impl Model for TopicPost {

    fn table() -> String { String::from("topic_posts") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
           "INSERT INTO post_topics
            (post_id, topic_id, link_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.post_id)
            .bind(&self.topic_id)
            .bind(&self.link_id)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)

    }
}
#[async_trait::async_trait]
impl Linked for TopicCategory {
    type Left = Topic;
    type Right = Category;

    fn link_id(self) -> Option<Id> {
        self.link_id
    }

    fn path() -> String {
        String::from("/{topic_id}/category/{category_id}")
    }

    /// Served at /topic/{topic_id}/category/{category_id}
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<(Id, Id)>| {
                let (topic_id, category_id) = id.into_inner();
                respond::ok(format!("GET /topic/{}/category/{}/hi", &topic_id, &category_id))
            }
            )
        );
    }
}
#[async_trait::async_trait]
impl Linked for TopicPost {
    type Left = Topic;
    type Right = Post;

    fn link_id(self) -> Option<Id> {
        self.link_id
    }
    fn left_id(self) -> Id {
        self.topic_id
    }
    fn right_id(self) -> Id {
        self.post_id
    }
    fn new_basic(left_id: Id, right_id: Id, link_id: Option<Id>) -> Self {
        Self {
            topic_id: left_id,
            post_id: right_id,
            link_id, ..Default::default()
        }

    }

    fn path() -> String {
        String::from("/{topic_id}/post/{post_id}")
    }

    /// Served at /topic/{topic_id}/post/{post_id}
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<(Id, Id)>| {
                let (topic_id, post_id) = id.into_inner();
                respond::ok(format!("GET /topic/{}/post/{}/hi", &topic_id, &post_id))
            }
            )
        );
    }
}

impl Topic {

    pub fn new(name: &str, description: Option<String>) -> Self {
        Self { id: Id::gen(), name: name.to_string(),
            description,
            created_at: now(),
            updated_at: now()
        }
    }

    pub async fn get_by_name(db: &PgPool, topic: String) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Topic>("SELECT * FROM topics WHERE name = $1")
            .bind(&topic)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn delete_by_name(db: &PgPool, topic: String) -> sqlx::Result<Option<Id>> {
        let res: Option<Id> = sqlx::query_scalar("DELETE FROM topics WHERE name = $1 returning id")
            .bind(topic)
            .fetch_optional(db).await?;
        Ok(res)
    }


    /* pub async fn add_link(self, db: &PgPool, link_id: Id) -> sqlx::Result<Self> {
        let top = sqlx::query_scalar("UPDATE topics (name) VALUES ($1) RETURNING id")
            .bind(link_id)
            .fetch_one(db).await?;
        Ok(Self { id: Some(top), ..self }) */

    // }

    pub async fn get_linked<T>(self, db: &PgPool, entity: &str, entity_id: Id) -> sqlx::Result<Vec<T>> {
        Ok(vec![])
    }

    pub async fn insert_alt(self, db: &PgPool) -> anyhow::Result<Topic> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO topics (name) VALUES ($1) RETURNING id")
            .bind(&self.name)
            .fetch_one(db).await?;
        Ok(res)

    }

    pub async fn post_new_thread(self, db: &PgPool, user_id: Id, content: String, image: Option<String>, feeling: Option<Feeling>) -> sqlx::Result<Post> {
        let post = Post::new(user_id, content, None, image, feeling)
            .insert(db).await?;
        Ok(post)
    }

    pub async fn get_all_threads(self, db: &PgPool) -> anyhow::Result<Vec<Post>> {
        let res = sqlx::query_as::<Postgres, Post>("
            SELECT * FROM posts
            INNER JOIN topic_posts
            ON posts.id = topic_posts.post_id
            WHERE topic_posts.topic_id = $1
            ")
            .bind(&self.id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn add_new_category(self, db: &PgPool, category: Category)
        -> anyhow::Result<Self> {
            let category = category.insert(db).await?;
            let res = sqlx::query_as::<Postgres, Self>("
                    INSERT INTO topic_categories
                    (category_id,topic_id, score)
                    VALUES ($1, $2, $3)
                    RETURNING *
                ")
                .bind(category.id)
                .bind(self.id)
                .bind(0.0)
                .fetch_one(db).await?;
            Ok(res)
    }

    pub async fn add_new_vote(self, db: &PgPool, vote: TopicVote)
        -> anyhow::Result<Topic> {
            let vote = vote.insert(db).await?;
            let res = sqlx::query_as::<Postgres, Self>("
                    INSERT INTO topic_votes
                    (topic_id,user_id, is_for, description, feeling)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING *
                ")
                .bind(self.id)
                .bind(vote.id)
                .bind(vote.is_for)
                .bind(vote.description)
                .bind(vote.feeling)
                .fetch_one(db).await?;
            Ok(res)
    }
}


impl TopicVote {

    pub fn new(user_id: Id, topic_id: Id, is_for: bool, feeling: Option<Feeling>, description: Option<String>) -> Self {
        Self {
            feeling, is_for, user_id, topic_id, id: Id::gen(), description,
            created_at: now(), updated_at: now()
        }
    }
    pub async fn linked_to_topic(db: &PgPool, topic_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM topic_votes WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
}

impl TopicCategory {

    pub fn new(user_id: Id, category_id: Id, topic_id: Id, score: Option<f64>) -> Self {
        Self {
            user_id,
            link_id: None,
            category_id, topic_id, score: score.unwrap_or(0.0), id: Id::gen(),
            created_at: now(), updated_at: now()
        }
    }

    pub async fn get_scores_between(db: &PgPool, topic_id: Id, category_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM topic_categories
            WHERE topic_id = $1
                  category_id = $2
        ")
            .bind(topic_id)
            .bind(category_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_all_by_user(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM topic_categories WHERE user_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn linked_to_topic(
        db: &PgPool, topic_id: Id
    ) -> sqlx::Result<Vec<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM topic_categories WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn update_score(
        db: &PgPool, user_id: Id, topic_id: Id, category_id: Id, score: f64
    ) -> sqlx::Result<Option<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE topic_categories
            SET score = $1
            WHERE user_id = $2, topic_id = $3, category_id = $4
            RETURNING *
            ")
            .bind(score)
            .bind(user_id)
            .bind(topic_id)
            .bind(category_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicContextData {
    pub user: Option<String>,
    pub found_in: Option<String>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicData {
    pub name: String,
    pub context: TopicContextData,
    pub created: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct TopicUserSubscriber {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "primary")]
    pub primary: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "blocked")]
    pub blocked: bool,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct TopicGroupSubscriber {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    #[serde(default = "Id::nil")]
    pub group_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "primary")]
    pub primary: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "blocked")]
    pub blocked: bool,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct TopicRelation {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub topic1_id: Id,
    #[serde(default = "Id::nil")]
    pub topic2_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct TopicLink {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "primary")]
    pub primary: bool,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

// #[derive(Message)]
// #[rtype(result = TopicReaction)]
// pub enum TopicInteraction {
//     NewVisit(TopicVisit),
//     NewPost(TopicPost),
//     NewSubscriber(TopicNewSubscriber),
//     NewMention(TopicMention),
// }

// #[derive(Message)]
// #[rtype(result = TopicReaction)]
// pub enum TopicLinkEvent {
//     UserLink(User),
//     RecordLink(Record),
//     GroupLink(Group),
//     ItemLink(Item),
//     FieldLink(Field),
//     TopicLink(Topic),
//     PostLink(String),
//     UnlinkedUser(User),
//     UnlinkedTopic(Topic),
// }

// pub enum TopicReaction {
//     Broadcast(Topic),
//     BroadcastOther(Topic),
//     BroadcastUser(User),
//     BroadcastPost(TopicPost),
//     BroadcastMention(TopicMention),
//     ParseText(String),
//     TrackLinked(Link),
//     DoNothing,
// }

impl Actor for Topic {
    type Context = Context<Self>;

    #[inline]
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Topic has started tracking");
    }

    #[inline]
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Topic has stopped tracking");
    }
}


// impl Handler<TopicLinkEvent> for Topic {
//     type Result = TopicReaction;
//     fn handle(&mut self, msg: TopicInteraction, _ctx: &mut Context<Self>) -> Self::Result {
//         println!("Topic {} has noticed got a new link event", self.name);
//         match msg {
//             TopicLinkEvent::PostLink(post) => TopicReaction::ParseText(post),
//             _ => TopicReaction::DoNothing,
//         }
//     }
// }
// impl Handler<TopicInteraction> for Topic {
//     type Result = TopicReaction;

//     fn handle(&mut self, msg: TopicInteraction, _ctx: &mut Context<Self>) -> Self::Result {
//         println!("Topic {} has noticed an interaction", self.name);
//         match msg {
//             TopicInteraction::NewPost(post) => TopicReaction::BroadcastPost(item),
//             TopicInteraction::NewMention(mention) => TopicReaction::BroadcastMention(mention),
//             TopicInteraction::NewVisit(link) => TopicReaction::Broadcast(self),
//             TopicInteraction::NewSubscriber(link) => TopicReaction::Broadcast(self),
//         }

//     }

// }

// impl<A, M> TopicResponse<A, M> for TopicReaction
// where
//     A: Actor,
//     M: Message<Result = TopicReaction>
// {
//     fn handle<R: RecordEvoChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
//         if let Some(tx) = tx {
//             tx.send(self)
//         }
//     }
// }

// // impl Handler<RecordEvent> for Topic {
// // }

// pub struct TopicLinkEstablishment {
//     id: Option<Id>,
//     link_id: Option<Id>,
//     user: User,
//     created_at: NaiveDate,
// }

// pub struct TopicMention {
//     user: User,
//     to: Option<User>,
//     context: Option<String>,
//     created_at: NaiveDate,
// }

// pub struct TopicNewSubscriber {
//     user: User,
//     created_at: NaiveDate,
// }

// pub struct TopicPost {
//     user: User,
//     content: String,
//     created_at: NaiveDate,
// }

// pub struct TopicVisit {
//     user: User,
//     created_at: NaiveDate,
// }


//     pub async fn notify_trending() -> () {
//         self.subscribers.push()
//     }
// }

// pub struct TopicSubscription {
//     subscriber: TopicSubscriber,
//     topic_subscriptions: HashMap<Uuid, Recipient<Topic>>, //confidence> recipient
//     created_at: NaiveDateTime,
//     updated_at: NaiveDateTime,
// }

// pub enum TopicSubscriber {
//     Record(Record),
//     User(User),
//     Item(Item),
//     Field(Field),
//     Group(Group),
//     LinkedTo(Link)
// }

// pub enum TopicSubscriptionResponse {
//     SubscribeTopic(Topic),
//     DistanceTopic(Topic),
//     UpdateConfidence(Uuid),
//     TrackRecord(Record),
//     TrackUser(User),
//     TrackLinked(Link),
//     TrackItem(Item),
//     TrackField(Field),
//     TrackTopic(Topic),
//     DoNothing,
// }

// impl Actor for TopicSubscription {
//     type Context = Context<Self>;
// }

// impl Handler<TopicLinkEvent> for TopicSubscription {
//     type Result = TopicSubscriptionResponse;

//     fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result {
//         let incr_confidence = |decr_factor: f64, topics: HashMap<Uuid, Recipient<Topic>>| {
//             let mut new_confidence = HashMap::new::<Uuid, Recipient<Topic>>();
//             for (confidence, topic_recp) in topics.drain() {
//                 new_confidence.insert(confidence*decr_factor as Uuid, topic_recp);
//             }
//             return new_confidence;
//         };
//         match msg {
//             TopicLinkEvent::TopicLink(topic) => TopicSubscriptionResponse::SubscribeTopic(topic),
//             TopicLinkEvent::UnlinkedTopic(topic) => TopicSubscriptionResponse::DistanceTopic(topic),
//             _ => TopicSubscriptionResponse::DoNothing,
//         }
//     }
// }


fn primary() -> bool { false }
fn blocked() -> bool { false }
