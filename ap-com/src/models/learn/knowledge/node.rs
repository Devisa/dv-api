use actix::*;
use crate::models::{
    user::{User, UserData, UserRelationData, UserRelation},
    item::{Item, ItemData, ItemRelation, ItemRelationData},
    record::{Record, RecordRelation, },
};
use petgraph::{
    prelude::*,
    algo::{dijkstra, astar}
};

#[derive(Debug, Default)]
pub struct KnowldgeNode {

}

