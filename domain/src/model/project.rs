use diesel::prelude::*;
use uuid::Uuid;

use crate::model::user::User;
use crate::schema::{projects, projects_users};

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    id: Uuid,
    pub name: String,
    pub description: Option<String>,
    archived: bool,
    client_id: Uuid,
}

impl Project {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn client_id(&self) -> Uuid {
        self.client_id
    }

    pub fn archived(&self) -> bool {
        self.archived
    }
}

#[derive(Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(User))]
#[diesel(table_name = projects_users)]
#[diesel(primary_key(project_id, user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectUser {
    project_id: Uuid,
    user_id: Uuid,
}

impl ProjectUser {
    pub fn project_id(&self) -> Uuid {
        self.project_id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}
