use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use super::User;
use crate::schema::profiles;

#[derive(Queryable, Insertable, Selectable, Identifiable, Associations)]
#[diesel(check_for_backend(Pg))]
#[diesel(belongs_to(User))]
pub struct Profile {
    id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub description: Option<String>,
    /// The id of the user the profile belongs to.
    user_id: Uuid,
}

impl Profile {
    // We don't want to accidentally change the user's id.
    pub fn id(&self) -> Uuid {
        self.id
    }

    // We don't want to accidentally change which user the profile belongs to.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
