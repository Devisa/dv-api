pub mod msg;
pub mod node;
pub mod edge;

use crate::models::{

    user::{User, UserData, UserRelationData, UserRelation},
    item::{Item, ItemData, ItemRelation, ItemRelationData},
    record::{Record, RecordRelation, },
};
use petgraph::{
    prelude::*,
    algo::{dijkstra, astar}
};

#[derive(Debug, Clone, )]
pub struct KnowledgeGraph<E, R> {
    graph: Graph<E, R>,
}

impl<E, R> KnowledgeGraph<E, R> {

    pub fn new(entity: E, relation: R) -> Self {
        let mut graph: Graph<E, R> = Graph::new();
        Self { graph }
    }
}

impl KnowledgeGraph<ItemData, ItemRelationData> {

    pub fn new_item(item: ItemData, relation: ItemRelationData) -> Self {
       let mut graph: Graph<ItemData, ItemRelationData> = Graph::new();
       Self { graph }

    }
}

impl KnowledgeGraph<UserData, UserRelationData> {

    pub fn new_user(item: UserData, relation: UserRelationData) -> Self {
       let mut graph: Graph<UserData, UserRelationData> = Graph::new();
       Self { graph }
    }
}
