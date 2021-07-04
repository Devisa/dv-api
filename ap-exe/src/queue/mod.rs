pub mod msg;
pub mod worker;

use actix::prelude::*;
use crossbeam_queue::SegQueue;
use crate::{
    queue::msg::{Push, Pop},
    error::PopError,
};

pub trait Queueable<T: Default> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Result<T, PopError>;
}

#[derive(Default)]
pub struct TaskQueue<T: Default> {
    queue: SegQueue<T>
}
impl<T: Default> Queueable<T> for TaskQueue<T> {
    fn push(&mut self, item: T) {
        self.queue.push(item);
    }
    fn pop(&mut self) -> Result<T, PopError> {
        self.queue.pop().ok_or(PopError)
    }
}
impl<T: Default + Sized + Unpin + 'static> Actor for TaskQueue<T> {
    type Context = Context<Self>;
}
impl<T: Default + Sized + Unpin + 'static> Handler<Push<T>> for TaskQueue<T> {
    type Result = ();
    fn handle(&mut self, msg: Push<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let item = msg.item;
        self.push(item);
    }
}
impl<T: Default + Sized + Unpin + 'static> Handler<Pop<T>> for TaskQueue<T> {
    type Result = Result<T, PopError>;
    fn handle(&mut self, _: Pop<T>, _ctx: &mut Context<Self>) -> Self::Result {
        self.queue.pop().ok_or(PopError)
    }
}
impl<T: Default + Sized + Unpin + 'static> Supervised for TaskQueue<T> {}
impl<T: Default + Sized + Unpin + 'static> SystemService for TaskQueue<T> {}
