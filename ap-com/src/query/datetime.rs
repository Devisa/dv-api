use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelativeDate {
    Before(NaiveDateTime),
    DayOf(NaiveDateTime),
    After(NaiveDateTime),
    Between(NaiveDateTime, NaiveDateTime),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateFilter {
    pub not: bool,
    pub rel_date: RelativeDate,
}
