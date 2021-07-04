use uuid::Uuid;
use actix::prelude::*;
use crate::{
    types::{Id, Status, now, private},
    rel::link::{Linked, LinkedTo},
    models::{Link, Model, field::{Field, FieldKind}}
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Item {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Item {
    fn table() -> String { String::from("items") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO items
            (id, name, user_id, private, status, description,
            image, cover_image, created_at, updated_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9, $10) RETURNING id
            ")
            .bind(&self.id)
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
        Ok(res)
    }
}
#[async_trait::async_trait]
impl Model for ItemField {
    fn table() -> String { String::from("item_fields") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO item_fields
            (item_id, field_id, link_id, name, description, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ")
            .bind(self.item_id)
            .bind(self.field_id)
            .bind(self.link_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }

}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    //pub user: UserData,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemRelation {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Id::nil")]
    pub item1_id: Id,
    #[serde(default = "Id::nil")]
    pub item2_id: Id,
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

#[derive(PartialEq, Debug, FromRow, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemField {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Id::nil")]
    pub item_id: Id,
    #[serde(default = "Id::nil")]
    pub field_id: Id,
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
impl Actor for Item {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("ITEM ACTOR STARTED: {:?}", self.id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("ITEM ACTOR STOPPED: {:?}", self.id);
    }
}

impl Default for ItemField {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            field_id: Id::nil(),
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
impl LinkedTo<Item> for Field {
    type LinkModel = ItemField;
}
#[async_trait::async_trait]
impl LinkedTo<Field> for Item {
    type LinkModel = ItemField;
}

#[async_trait::async_trait]
impl Linked for ItemField {
    type Left = Item;
    type Right = Field;

    fn new_basic(left_id: Id, right_id: Id, link_id: Option<Id>) -> Self {
        Self {
            item_id: left_id, field_id: right_id, link_id,
            ..Default::default()
        }
    }

    fn link_id(self) -> Option<Id> { self.link_id }
    fn left_id(self) -> Id { self.item_id }
    fn right_id(self) -> Id { self.field_id }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRelationData {
    //pub user: UserData,
    pub item1: ItemData,
    pub item2: ItemData,
    // pub link: LinkData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for Item {
    fn default() -> Self {
        Item {
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

impl ItemField {
    pub fn new(item_id: Id, field_id: Id, link_id: Option<Id>, name: Option<String>, description: Option<String>) -> Self {
        Self {
            field_id, item_id, link_id, name, description, ..Default::default()
        }

    }

}

impl Item {

    pub fn new(name: String, user_id: Id) -> Self {
        Self {
            user_id, name, ..Default::default()
        }
    }

    pub async fn update_by_id(db: &PgPool, id: Id, i: Item)
        -> anyhow::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE     items
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
            .bind(&i.name)
            .bind(&i.description)
            .bind(&i.private)
            .bind(&i.image)
            .bind(&i.status)
            .bind(&i.cover_image)
            .bind(now())
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_by_user(db: &PgPool, user_id: Id) -> anyhow::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Item>("SELECT * FROM items WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn add_new_field(self, db: &PgPool, name: String, kind: FieldKind) -> anyhow::Result<ItemField> {
        let field = Field::new(name, kind, self.user_id).insert(&db).await?;
        let item_field = ItemField::new_basic(self.id, field.id, None).insert(&db).await?;
        Ok(item_field)
    }

    pub async fn add_existing_item(self, db: &PgPool, field_id: Id) -> anyhow::Result<ItemField> {
        let field = Field::get(&db, field_id).await?;
        if let Some(field) = field {
            let item_field = ItemField::new_basic(self.id, field.id, None).insert(&db).await?;
            Ok(item_field)
        } else {
            return Err(anyhow::anyhow!("Item does not exist"));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemName {
    pub name: String
}

