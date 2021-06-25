use sqlx::{FromRow, Postgres};

use crate::types::Model;

pub struct Query<T: Model> {
    table: T
}

/* pub struct Query {

}
 */
/* pub fn get_all<'r, T: Model + FromRow + 'static>(model: T) -> sqlx::query::QueryScalar<'r, Postgres, T> {
    let q_str = format!("SELECT * FROM {}", T::table());
    sqlx::query_as::<Postgres, T>(&q_str)
} */
