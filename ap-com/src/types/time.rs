use std::ops;
use sqlx::Type;
use chrono::{Duration, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Type, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[sqlx(transparent, type_name = "expiration")]
pub struct Expiration(NaiveDateTime);

impl Default for Expiration {
    #[inline]
    fn default() -> Self {
        Self::two_days()
    }
}
impl ops::Deref for Expiration {
    type Target = NaiveDateTime;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for Expiration {
    #[inline]
    fn deref_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.0
    }
}

impl Expiration {

    pub fn two_days() -> Self {
        let today = Utc::now().naive_utc();
        let two_days = today.checked_add_signed(Duration::days(2))
            .expect("Invalid datetime?");
        return Self(two_days);
    }

    #[inline]
    pub fn with_days(days: i64) -> Self {
        Self(Utc::now().naive_utc() + Duration::days(days))
    }

    #[inline]
    pub fn with_hours(hours: i64) -> Self {
        Self(Utc::now().naive_utc() + Duration::hours(hours))
    }

    #[inline]
    pub fn secs_left(&self) -> u32 {
        (self.get() - Utc::now().naive_utc())
            .num_seconds() as u32
    }

    #[inline]
    pub fn hours_left(&self) -> u32 {
        (self.get() - Utc::now().naive_utc())
            .num_hours() as u32
    }

    #[inline]
    pub fn time_left(&self) -> Duration {
        self.get() - Utc::now().naive_utc()
    }

    #[inline]
    pub fn get(&self) -> NaiveDateTime {
        self.0
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ExpirationQuery {
    pub weeks: Option<u16>,
    pub days: Option<u16>,
    pub hours: Option<u16>,
    pub mins: Option<u16>,
}
