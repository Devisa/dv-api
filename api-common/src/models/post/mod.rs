use crate::{
    types::{Status, now, private, Feeling},
    models::{
        topic::Topic,
        link::{LinkedTo, Linked},
        book::post::BookPost,
        Model
    },
};
use uuid::Uuid;
use sqlx::{FromRow, PgPool, Postgres, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    #[serde(default = "Uuid::new_v4", skip_serializing_if="Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responds_to_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeling: Option<Feeling>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupPost {
    #[serde(default = "Uuid::new_v4", skip_serializing_if="Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub group_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub post_id: Uuid,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Post {
    fn table() -> String { String::from("posts") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("INSERT INTO posts
            (id, user_id, title, content, image, status, private,
            created_at, updated_at, feeling, responds_to_id)
            VALUES ($1,, $2,, $3,, $4,, $5,, $6,, $7,, $8,, $9,, $10) RETURNING id")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.content)
            .bind(&self.image)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .bind(&self.feeling)
            .bind(&self.responds_to_id)
            .fetch_one(db).await?;
        Ok(res)
    }

}

impl GroupPost {

    pub fn new(group_id: Uuid, post_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id, post_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }
}

#[async_trait::async_trait]
impl Model for GroupPost {
    fn table() -> String { String::from("group_posts") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO group_posts (id, group_id, post_id, created_at, updated_at)
            VALUES ($1,, $2,, $3,, $4,, $5) RETURNING *")
            .bind(&self.id)
            .bind(&self.group_id)
            .bind(&self.post_id)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }

}

impl Post {

    pub fn new(user_id: Uuid, content: String, responds_to: Option<Uuid>, image: Option<String>, feeling: Option<Feeling>) -> Self {
        Self {
            id: Uuid::new_v4(),
            responds_to_id: responds_to,
            feeling,
            image: None,
            user_id, content,
            private: true,
            status: Status::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }

    pub async fn get_in_topic(db: &PgPool, topic_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM posts
            INNER JOIN topic_posts ON topic_posts.post_id = posts.item_id
            INNER JOIN topics ON topics.id = topic_posts.topic_id
            WHERE topic_id =, $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_in_group(db: &PgPool, group_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM posts
            INNER JOIN group_posts ON group_posts.post_id = posts.item_id
            INNER JOIN groups ON groups.id = group_posts.group_id
            WHERE group_id =, $1
        ")
            .bind(group_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_all_by_user(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM posts WHERE user_id =, $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn insert_group(self, db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM posts WHERE user_id =, $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn add_image(self, db: &PgPool, image: String) -> sqlx::Result<Self> {
        let post = Self { image: Some(image), ..self };
        // TODO update in db
        return Ok(post);
    }
    pub async fn make_private(self) -> Self {
        return Self { private: true, ..self };
    }

    pub async fn add_to_book(self, db: &PgPool, book_id: Uuid, link_id: Option<Uuid>) -> sqlx::Result<Self> {
        let entry = BookPost {
            id: Uuid::new_v4(),
            book_id,
            post_id: self.id,
            link_id,
        };
        let res = entry.insert(&db).await?;
        Ok(self)
    }
    pub async fn add_to_topic(self, db: &PgPool, topic_id: Uuid, link_id: Option<Uuid>) -> sqlx::Result<TopicPost> {
        let entry = TopicPost::new(self.id, topic_id, link_id)
            .insert(db).await?;
        Ok(entry)
    }

    pub async fn insert_reply(self, db: &PgPool, post_id: Uuid) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
        INSERT INTO posts (id, user_id, title, content,
            image, status, private, created_at, updated_at, feeling, responds_to_id)
        VALUES $1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.content)
            .bind(&self.image)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .bind(&self.feeling)
            .bind(&post_id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn add_to_group(db: &PgPool, group_id: Uuid, post_id: Uuid) -> sqlx::Result<GroupPost> {
        let group_post = GroupPost::new(group_id, post_id).insert(db).await?;
        Ok(group_post)
    }

    pub async fn get_topics(self, db: &PgPool) -> sqlx::Result<Vec<TopicPost>> {
        let res = sqlx::query_as::<Postgres, TopicPost>("
            SELECT * FROM topic_posts
            WHERE post_id = $1
            ")
            .bind(&self.id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_feeling_responses(self, db: &PgPool) -> sqlx::Result<Vec<PostFeelingResponse>> {
        let res = sqlx::query_as::<Postgres, PostFeelingResponse>("
            SELECT * FROM post_feeling_responses
            WHERE post_id = $1
            ")
            .bind(&self.id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn add_feel_reply(self, db: &PgPool, user_id: Uuid, feeling: Feeling) -> sqlx::Result<PostFeelingResponse> {
        let fr = PostFeelingResponse::new(self.id, user_id, feeling)
            .insert(db).await?;
        Ok(fr)
    }
    pub async fn add_topic(self, db: &PgPool, topic_id: Uuid, link_id: Option<Uuid>) -> sqlx::Result<TopicPost> {
        let fr = TopicPost::new(self.id, topic_id, link_id)
            .insert(db).await?;
        Ok(fr)
    }
    pub async fn add_group(self, db: &PgPool, group_id: Uuid) -> sqlx::Result<GroupPost> {
        let res = sqlx::query_as::<Postgres, GroupPost>("
            INSERT INTO group_posts (group_id, post_id)
            VALUES ($1, $2)
            RETURNING *
        ")
            .bind(group_id)
            .bind(self.id)
            .fetch_one(db).await?;
        Ok(res)
    }

}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopicPost {
    #[serde(default = "Uuid::new_v4", skip_serializing_if="Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub post_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub topic_id: Uuid,
    #[serde(skip_serializing_if="Option::is_none")]
    pub link_id: Option<Uuid>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
impl Default for TopicPost {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id: Uuid::nil(),
            topic_id: Uuid::nil(),
            link_id: None,
            created_at: now(),
            updated_at: now(),
        }
    }
}
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostFeelingResponse {
    #[serde(default = "Uuid::new_v4", skip_serializing_if="Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub post_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub user_id: Uuid,
    pub feeling: Feeling,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
impl PostFeelingResponse {
    pub fn new(post_id: Uuid, user_id: Uuid, feeling: Feeling,) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id: Uuid::nil(),
            user_id: Uuid::nil(),
            feeling,
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl TopicPost {

    pub fn new(post_id: Uuid, topic_id: Uuid, link_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id: Uuid::nil(),
            topic_id: Uuid::nil(),
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
impl Model for PostFeelingResponse {
    fn table() -> String { String::from("post_feeling_responses") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO post_feeling_responses
            (post_id, user_id, feeling, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.post_id)
            .bind(&self.user_id)
            .bind(&self.feeling)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}
#[async_trait::async_trait]
impl Linked for TopicPost {
    type Left = Topic;
    type Right = Post;
    fn new_basic(left_id: Uuid, right_id: Uuid, link_id: Option<Uuid>) -> Self {
        Self {
            topic_id: left_id,
            post_id: right_id,
            link_id, ..Default::default()
        }

    }
    fn link_id(self) -> Option<Uuid> {
        self.link_id
    }
    fn left_id(self) -> Uuid {
        self.topic_id
    }
    fn right_id(self) -> Uuid {
        self.post_id
    }
}
impl LinkedTo<Topic> for Post {
    type LinkModel = TopicPost;
}
impl LinkedTo<Post> for Topic {
    type LinkModel = TopicPost;
}
