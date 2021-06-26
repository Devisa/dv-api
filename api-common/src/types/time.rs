use derive_more::{From, Display, AsMut, AsRef, FromStr, Error};
use chrono::{Duration, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(sqlx::Type, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[sqlx(transparent, type_name = "expiration")]
pub struct Expiration(chrono::NaiveDateTime);

impl Expiration {

    pub fn two_days() -> Self {
        let today = Utc::now().naive_utc();
        let two_days = today.checked_add_signed(Duration::days(2))
            .expect("Invalid datetime?");
        return Self(two_days);
    }

    pub fn with_days(days: i64) -> Self {
        Self(Utc::now().naive_utc() + Duration::days(days))
    }

    pub fn with_hours(hours: i64) -> Self {
        Self(Utc::now().naive_utc() + Duration::hours(hours))
    }

    pub fn secs_left(&self) -> u32 {
        (self.get() - Utc::now().naive_utc())
            .num_seconds() as u32
    }

    pub fn hours_left(&self) -> u32 {
        (self.get() - Utc::now().naive_utc())
            .num_hours() as u32
    }

    pub fn time_left(&self) -> Duration {
        self.get() - Utc::now().naive_utc()
    }

    pub fn get(&self) -> NaiveDateTime {
        self.0
    }
}
