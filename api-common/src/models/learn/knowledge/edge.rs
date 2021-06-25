use std::rc::Rc;

use petgraph::{
    prelude::*,
    unionfind::UnionFind,
    Direction, EdgeDirection, Directed, Incoming, Outgoing,
};

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

use super::{msg::KnowledgeGraphNode, KnowledgeGraph};

#[derive(Debug,)]
pub struct KnowledgeGraphEdge<'a, E, R> {
    edge_id: u32,
    direction: EdgeDirection,
    relation: Box<&'a R>,
    graph: Rc<KnowledgeGraph<E,R>>,
}

impl<'a:, E, R> Actor for KnowledgeGraphEdge<'a, E, R>
where
    'a: 'static,
    E: 'static,
    R: 'static
{
    type Context = Context<Self>;
}

impl<'a, E, R> Supervised for KnowledgeGraphEdge<'a, E, R>
where
    'a: 'static,
    E: 'static,
    R: 'static
{
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        println!("Restarting node {}", self.edge_id);
    }

}
