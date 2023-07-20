use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::profiles;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub description: Option<String>,
    /// The id of the user the profile belongs to.
    user_id: Uuid,
}

impl Profile {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
