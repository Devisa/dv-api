use chrono::{DateTime, NaiveDateTime};

pub struct Feel {
    pub id: Option<i32>,
    pub feeling_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

pub struct Feeling {
    pub id: Option<i32>,
    pub feeling_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,

}
