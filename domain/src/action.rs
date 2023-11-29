use diesel_async::AsyncPgConnection;

pub mod project;
pub mod task;

pub struct Context<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> Context<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}
