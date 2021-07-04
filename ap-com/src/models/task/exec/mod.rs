use actix::prelude::*;
use crate::{Id, Db, Model};
use derive_more::{AsRef, AsMut, Display, From};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool, types::Json};
use uuid::Uuid;
use super::{TaskBookExecStatus, TaskStepExecStatus, condition::{TaskBookExecCondition, TaskStepExecCondition}};
use crate::Status;


pub struct TaskBookExecution {
    pub id: Id,
    pub task_book_id: Id,
    pub condition: TaskBookExecCondition,
    pub status: TaskBookExecStatus,
    pub started: NaiveDateTime,
    pub finished: NaiveDateTime,
}
pub struct TaskStepExecution {
    pub id: Id,
    pub task_step_id: Id,
    pub condition: TaskStepExecCondition,
    pub status: TaskStepExecStatus,
    pub started: NaiveDateTime,
    pub finished: NaiveDateTime,
}

/* pub struct StepExecutionData {
    pub data: Json,
} */
