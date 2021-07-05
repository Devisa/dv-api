use crate::models::user::User;
use actix_web::{HttpResponse, web::{get, post, Path, ServiceConfig}};
use uuid::Uuid;
use crate::{util::respond, models::Post, Model, Linked, LinkedTo, GroupRole, Id, Status, now, private};
use actix::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc},
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "String::new")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupUser {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "Id::nil")]
    pub group_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<GroupRole>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupPost {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub group_id: Id,
    #[serde(default = "Id::nil")]
    pub post_id: Id,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
impl GroupPost {

    pub fn new(group_id: Id, post_id: Id) -> Self {
        Self {
            id: Id::gen(),
            group_id, post_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }
}
#[async_trait::async_trait]
impl Model for Group {

    #[inline]
    fn table() -> String { String::from("groups") }

    #[inline]
    fn path() -> String { String::from("/group") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /group/hi".to_string())))
            .service(<Group as LinkedTo<User>>::scope())
            .service(<Group as LinkedTo<Post>>::scope())
            .service(<GroupUser as Linked>::scope())
            .service(<GroupPost as Linked>::scope());
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Group> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO groups (id, user_id, name, description, image, cover_image, private, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            ")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.image)
            .bind(&self.cover_image)
            .bind(&self.private)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl Model for GroupPost {
    #[inline]
    fn table() -> String { String::from("group_posts") }
    #[inline]
    fn path() -> String { String::from("/post") }

    fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET group/post/hi".to_string())));

    }
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



impl Actor for Group {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("GROUP ACTOR STARTED: {:?}", self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("GROUP ACTOR STOPPED: {:?}", self.id);
    }
}


#[async_trait::async_trait]
impl Model for GroupUser {

    fn table() -> String { String::from("group_users") }
    fn path() -> String { String::from("/user") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /group/user/hi".to_string())));
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO group_users (id, user_id, group_id, link_id, name, description,
                    role, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.group_id)
            .bind(&self.link_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.role)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }

}


impl Default for GroupUser {
    fn default() -> Self {
        GroupUser {
            id: Id::gen(),
            user_id: Id::nil(),
            group_id: Id::nil(),
            ..Default::default()
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Group {
            id: Id::gen(),
            user_id: Id::nil(),
            name: String::new(),
            description: None,
            private: false,
            status: Status::Active,
            image: None,
            cover_image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl GroupUser {
}

impl Group {
    pub async fn add_member(db: &PgPool, group_id: Id, user_id: Id) -> anyhow::Result<GroupUser> {
        let gu = GroupUser { user_id, group_id, ..Default::default() }
            .insert(db).await?;
        Ok(gu)
    }

    pub async fn update_name(db: &PgPool, group_id: Id, name: String) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Group>("
            UPDATE groups
            SET name = $1
            WHERE id = $2
            RETURNING *")
            .bind(name)
            .bind(group_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    pub async fn update_description(db: &PgPool, group_id: Id, description: String) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Group>("
            UPDATE groups
            SET description = $1
            WHERE id = $2
            RETURNING *")
            .bind(description)
            .bind(group_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    pub async fn get_all_by_user(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Group>("SELECT * FROM Groups WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn delete(db: &PgPool, id: Id) -> sqlx::Result<Option<Id>> {
        let res = sqlx::query_scalar("
            DELETE FROM groups WHERE id = $1 RETURNING id
            ")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn get_all_users(db: &PgPool, id: Id) -> sqlx::Result<Vec<User>> {
        let res = sqlx::query_as::<Postgres, User>("
            SELECT * FROM users
            INNER JOIN group_users ON group_users.user_id = users.id
            INNER JOIN groups ON groups.id = group_users.group_id
            WHERE groups.id = $1
            ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn update_by_id(db: &PgPool, id: Id, g: Group)
        -> anyhow::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE     groups
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
            .bind(&g.name)
            .bind(&g.description)
            .bind(&g.private)
            .bind(&g.image)
            .bind(&g.status)
            .bind(&g.cover_image)
            .bind(now())
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn insert(self, db: &PgPool) -> anyhow::Result<Group> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO groups (user_id, name, description, image, cover_image, private, status) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            ")
            .bind(&self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.image)
            .bind(&self.cover_image)
            .bind(&self.private)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }


}

impl Default for GroupPost {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            group_id: Id::nil(),
            post_id: Id::nil(),
            updated_at: now(),
            created_at: now(),
        }
    }
}

#[async_trait::async_trait]
impl LinkedTo<User> for Group {
    type LinkModel = GroupUser;

    fn path() -> String { String::from("/{group_id}/user") }

    /// Served at /group/{id}/user
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|group_id: Path<Id>| respond::ok(format!("GET /group/{}/user/hi", &group_id))));
    }
}
#[async_trait::async_trait]
impl LinkedTo<Post> for Group {
    type LinkModel = GroupPost;

    fn path() -> String { String::from("/{group_id}/post") }

    /// Served at /group/{id}/post
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|post_id: Path<Id>| respond::ok(format!("GET /group/{}/post/hi", &post_id))));
    }
}

#[async_trait::async_trait]
impl Linked for GroupUser {
    type Left = Group;
    type Right = User;

    fn path() -> String { String::from("/{group_id}/user/{user_id}") }
    fn link_id(self) -> Option<Id> { self.link_id }
    fn left_id(self) -> Id { self.group_id }
    fn right_id(self) -> Id { self.user_id }

    /// Served at /group/{id}/user/{user_id}
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|ids: Path<(Id, Id)>| {
                let (group_id, user_id) = ids.into_inner();
                respond::ok(format!("GET /group/{}/user/{}/hi", &group_id, &user_id))
            }));
    }
}
#[async_trait::async_trait]
impl Linked for GroupPost {
    type Left = Group;
    type Right = Post;

    fn path() -> String { String::from("/{group_id}/post/{post_id}") }
    fn link_id(self) -> Option<Id> { None }
    fn left_id(self) -> Id { self.group_id }
    fn right_id(self) -> Id { self.post_id }

    /// Served at /group/{id}/user/{user_id}
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|ids: Path<(Id, Id)>| {
                let (group_id, user_id) = ids.into_inner();
                respond::ok(format!("GET /group/{}/user/{}/hi", &group_id, &user_id))
            }));
    }
}
