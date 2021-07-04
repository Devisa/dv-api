use std::marker::PhantomData;
use actix::prelude::*;
use crate::error::PopError;

#[derive(Message)]
#[rtype(result="()")]
pub struct Push<T> {
    pub item: T,
}

#[derive(Debug, Clone)]
pub struct Pop<T> {
    pub data: PhantomData<T>,
}

impl<T> Push<T> {
    pub fn new(item: T) -> Push<T> {
        Push { item }
    }
}

impl<T> Pop<T> {
    pub fn new() -> Pop<T> {
        Pop { data: PhantomData }
    }
}

impl <T: 'static> Message for Pop<T> {
    type Result = Result<T, PopError>;
}
