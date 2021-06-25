use actix::prelude::*;
use super::knowledge::KnowledgeGraph;
use crate::{
    db::Db,
    models::{
        link::{RecordItem, ItemField},
        user::{User, UserData},
        item::{Item, ItemData, ItemRelation, ItemRelationData},
        record::{Record, RecordRelation},
        link::Link,
    },
};
use petgraph::{
    prelude::*,
    graph::{Node, Edge, Graph},
    algo::{ astar, min_spanning_tree }
};


pub struct LearnUnitExplorer<E, R> {
    pub user: Box<User>,
    pub record: Box<Record>,
    pub data: KnowledgeGraph<E, R>,
    pub interval: u64,
}

impl<E, R> LearnUnitExplorer<E, R> {

    pub fn new() {
        let mut graph = Graph::<Item, Item, Directed>::new();
        let it1 = graph.add_node(Item::new("chris's health".into(), 1));
        let it2 = graph.add_node(Item::new("testman's wit".into(), 2));

    }

}
