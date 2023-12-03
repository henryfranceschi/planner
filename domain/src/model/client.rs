use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::clients;

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(check_for_backend(Pg))]
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
