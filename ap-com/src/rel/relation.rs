//! Defines the relation trait, which defines many-many links between
//! a model and another model of the same model type. I.e. a Relation
//! implemented for a User will only deal with the user type.

use crate::{Model, Id};

#[async_trait::async_trait]
pub trait Relates: Model {

    type Relates: Model;
    type Relation: Relation;

    fn new_basic(left_id: Id, right_id: Id) -> Self;

}

/// Defines a relation type between two of the same model type
/// ```
/// pub enum UserRelation {
///     FriendOf,
///     Blocks,
///     Follows
/// }
/// impl Relation for UserRelation {
///     ...
/// }
/// ```
#[async_trait::async_trait]
pub trait Relation {

}
