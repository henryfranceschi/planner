use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::clients;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Client {
    id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Client {
    pub fn id(&self) -> Uuid {
        self.id
    }
}
