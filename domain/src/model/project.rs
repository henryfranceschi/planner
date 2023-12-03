use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::user::User;
use crate::schema::{projects, projects_users};

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(check_for_backend(Pg))]
pub struct Project {
    id: Uuid,
    pub name: String,
    pub description: Option<String>,
    public: bool,
    archived: bool,
    client_id: Option<Uuid>,
}

impl Project {
    pub fn new(
        name: String,
        description: Option<String>,
        public: bool,
        archived: bool,
        client_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            public,
            archived,
            client_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn client_id(&self) -> Option<Uuid> {
        self.client_id
    }

    pub fn archived(&self) -> bool {
        self.archived
    }
}

#[derive(Queryable, Identifiable, Selectable, Associations)]
#[diesel(check_for_backend(Pg))]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(User))]
#[diesel(table_name = projects_users)]
#[diesel(primary_key(project_id, user_id))]
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
