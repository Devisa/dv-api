//! Defines a "has a" relationship -- i.e. a one-to-many relationship
//! For example, one field has many field values, and field targets
//!
use crate::{Model, Id};

pub trait Has<T>
where
    for<'a> Self: Model,
    for<'a> T: Model
{

}
