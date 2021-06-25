/*
use std::time::Duration;
use riker::actors::*;

fn main() {
    run();
}


fn run() {
    let sys = SystemBuilder::new().name("sys").create().unwrap();
    let a = sys.actor_of::<MyActor>("my-actor").unwrap();
    a.tell("Hello actor!".to_string(), None);
}

#[derive(Debug, Default)]
struct MyActor;
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self,
        _ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        _sender: Sender) {
        println!("received {}", msg);
    }

}

*/
use petgraph::graph::Graph;

fn main() {
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges(&[
        (origin, destination_1, 250),
        (origin, destination_2, 1099)
    ]);

    println!("{}", serde_json::to_string_pretty(&graph).unwrap());
}

