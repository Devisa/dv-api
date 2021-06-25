use std::marker::PhantomData;
use actix::{Addr, Context};
use actix::prelude::*;
use crate::queue::msg::{Pop, Push};
use crate::error::WorkerExecError;

use super::TaskQueue;

#[async_trait::async_trait]
pub trait QueueConsumer<T: Default + Sized + Unpin + 'static, W> {
    async fn exec(&self, task: T) -> Result<W, WorkerExecError>;
    fn get_queue(&self) -> Addr<TaskQueue<T>>;
    fn retry(&self, task: T) -> T;
    fn drop(&self, task: T);
    fn result(&self, result: W);
}

pub struct TaskWorker<T: Default + Sized + Unpin + 'static, W: Send + Sized + Unpin + 'static> {
    task: PhantomData<T>,
    result: PhantomData<W>
}
impl<T: Default + Sized + Unpin + 'static, W: Send + Sized + Unpin + 'static> Actor for TaskWorker<T, W> {
    type Context = Context<Self>;
}
impl<T: Copy + Default + Sized + Unpin + Send + 'static, W: Send + Sized + Unpin + 'static> TaskWorker<T, W>
where
    Self: QueueConsumer<T, W>
{
    pub fn new() -> TaskWorker<T, W> {
        TaskWorker {
            task: PhantomData,
            result: PhantomData
        }
    }
    pub async fn next(&self) {
        let queue = self.get_queue();
        if let Ok(ret) = queue.send(Pop::new()).await {
            match ret {
                Ok(task) => {
                    match self.exec(task).await {
                        Ok(res) => self.result(res),
                        Err(e) => match e {
                            WorkerExecError::Retryable => queue.do_send(Push::new(self.retry(task))),
                            WorkerExecError::NotRetryable => self.drop(task),
                        }
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
