use uuid::Uuid;
use actix::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres,
    prelude::*, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};
use crate::{
    types::{Status, now, private, Feeling},
    models::{
        Model,
        book::topic::TopicBook,
        post::{Post, TopicPost},
    }
};

#[derive(Debug, Clone, Serialize, Deserialize, )]
pub struct ScoreRequest {
    #[serde(default = "Uuid::new_v4")]
    pub user_id: Uuid,
    #[serde(default = "ScoreRequest::zero")]
    pub score: f64,
}

impl Default for ScoreRequest {
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            score: 0.0
        }
    }
}

impl ScoreRequest {
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
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Topic {
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

#[async_trait::async_trait]
impl Model for Category {
    fn table() -> String { String::from("categories") }
    fn id_str() -> String { String::from("category_id") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res =  sqlx::query_as::<Postgres, Self>("
            INSERT INTO categories (id, name, description)
            VALUES ($1, $2, $) RETURNING *")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.description)
            .fetch_one(db).await?;
        Ok(res)
    }
}
#[async_trait::async_trait]
impl Model for TopicCategory {
    fn table() -> String { String::from("topic_categories") }
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
    fn table() -> String { String::from("topic_votes") }
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

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicCategory {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub user_id: Uuid,
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub category_id: Uuid,
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub topic_id: Uuid,
    pub score: f64,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicVote {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub user_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub topic_id: Uuid,
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

impl Topic {

    pub fn new(name: &str, description: Option<String>) -> Self {
        Self { id: Uuid::new_v4(), name: name.to_string(),
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
    pub async fn delete_by_name(db: &PgPool, topic: String) -> sqlx::Result<Option<Uuid>> {
        let res: Option<Uuid> = sqlx::query_scalar("DELETE FROM topics WHERE name = $1 returning id")
            .bind(topic)
            .fetch_optional(db).await?;
        Ok(res)
    }


    /* pub async fn add_link(self, db: &PgPool, link_id: Uuid) -> sqlx::Result<Self> {
        let top = sqlx::query_scalar("UPDATE topics (name) VALUES ($1) RETURNING id")
            .bind(link_id)
            .fetch_one(db).await?;
        Ok(Self { id: Some(top), ..self }) */

    // }

    pub async fn get_linked<T>(self, db: &PgPool, entity: &str, entity_id: Uuid) -> sqlx::Result<Vec<T>> {
        Ok(vec![])
    }

    pub async fn insert_alt(self, db: &PgPool) -> anyhow::Result<Topic> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO topics (name) VALUES ($1) RETURNING id")
            .bind(&self.name)
            .fetch_one(db).await?;
        Ok(res)

    }

    pub async fn post_new_thread(self, db: &PgPool, user_id: Uuid, content: String, image: Option<String>, feeling: Option<Feeling>) -> sqlx::Result<Post> {
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

impl Category {

    pub fn new(name: String, description: Option<String>) -> Self {
        let cat = Self {
            name, id: Uuid::new_v4(), description,
            created_at: now(), updated_at: now()
        };
        cat
    }


    pub async fn get_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM categories")
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_by_id(db: &PgPool, id: Uuid) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn delete_by_id(db: &PgPool, id: Uuid) -> sqlx::Result<Option<Uuid>> {
        let res = sqlx::query_scalar("DELETE FROM categories WHERE id = $1 ")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn linked_to_topic(db: &PgPool, topic_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM categories
            INNER JOIN topic_categories
            ON categories.id = topic_categories.category_id
            WHERE topic_categories.topic_id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_posts(db: &PgPool, topic_id: Uuid) -> sqlx::Result<Vec<Topic>>
    {
        let res = sqlx::query_as::<Postgres, Topic>("
            SELECT * FROM topics
            INNER JOIN topic_posts ON posts.id = topic_posts.post_id
            INNER JOIN topics on topics.id = topic_posts.topic_id
            WHERE topics.id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
}

impl TopicVote {

    pub fn new(user_id: Uuid, topic_id: Uuid, is_for: bool, feeling: Option<Feeling>, description: Option<String>) -> Self {
        Self {
            feeling, is_for, user_id, topic_id, id: Uuid::new_v4(), description,
            created_at: now(), updated_at: now()
        }
    }
    pub async fn linked_to_topic(db: &PgPool, topic_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM topic_votes WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
}

impl TopicCategory {

    pub fn new(user_id: Uuid, category_id: Uuid, topic_id: Uuid, score: Option<f64>) -> Self {
        Self {
            user_id,
            category_id, topic_id, score: score.unwrap_or(0.0), id: Uuid::new_v4(),
            created_at: now(), updated_at: now()
        }
    }

    pub async fn get_scores_between(db: &PgPool, topic_id: Uuid, category_id: Uuid) -> sqlx::Result<Vec<Self>> {
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

    pub async fn get_all_by_user(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM topic_categories WHERE user_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn linked_to_topic(
        db: &PgPool, topic_id: Uuid
    ) -> sqlx::Result<Vec<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM topic_categories WHERE topic_id = $1")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn update_score(
        db: &PgPool, user_id: Uuid, topic_id: Uuid, category_id: Uuid, score: f64
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
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub topic_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Uuid>,
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
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub topic_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub group_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Uuid>,
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
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub topic1_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub topic2_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Uuid>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct TopicLink {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil")]
    pub topic_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Uuid>,
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

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Topic has started tracking");
    }

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
//     id: Option<Uuid>,
//     link_id: Option<Uuid>,
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
