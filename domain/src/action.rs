use async_trait::async_trait;
use diesel_async::AsyncPgConnection;
use uuid::Uuid;

use crate::{error::DomainError, model::User};

pub mod project;
pub mod task;

// TODO: currently many actions only require an id, an issue with this is that it is very easy
// to pass the wrong type of id. Think about possible solutions to this problem.

pub struct Context<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> Context<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }

    pub fn user(&self) -> Option<&User> {
        todo!()
    }
}

pub enum Operation {
    Read,
    ReadWrite,
}

#[async_trait]
pub trait Allow {
    async fn allow(ctx: &mut Context<'_>, id: Uuid, op: Operation) -> Result<(), DomainError>;
}
