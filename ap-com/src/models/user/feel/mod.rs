use crate::{Model, Db, Id};
use crate::now;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feel {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub feeling_id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feeling {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub feeling_id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,

}
