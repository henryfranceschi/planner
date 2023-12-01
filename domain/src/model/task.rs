use chrono::{DateTime, Utc};
use diesel::helper_types::{AsSelect, Find, FindBy, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use crate::error::DomainError;
use crate::model::user::User;
use crate::schema::{projects, tasks, tasks_users, users};

#[derive(Queryable, Insertable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Project))]
pub struct Task {
    id: Uuid,
    pub name: String,
    pub description: Option<String>,
    deadline: Option<DateTime<Utc>>,
    done: bool,
    project_id: Uuid,
}

type TaskUsersJoin = InnerJoin<FindBy<tasks_users::table, tasks_users::task_id, Uuid>, users::table>;
type RelatedUsers = Select<TaskUsersJoin, AsSelect<User, diesel::pg::Pg>>;

impl Task {
    pub fn new(
        name: String,
        description: Option<String>,
        deadline: Option<DateTime<Utc>>,
        done: bool,
        project_id: Uuid,
    ) -> Result<Self, DomainError> {
        if let Some(deadline) = deadline {
            Self::validate_deadline(&deadline)?;
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            deadline,
            done,
            project_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }

    pub fn set_deadline(&mut self, deadline: Option<DateTime<Utc>>) -> Result<(), DomainError> {
        if let Some(deadline) = deadline {
            Self::validate_deadline(&deadline)?;
        }

        self.deadline = deadline;

        Ok(())
    }

    fn validate_deadline(deadline: &DateTime<Utc>) -> Result<(), DomainError> {
        if deadline < &Utc::now() {
            Err(DomainError::Validation {
                model: "Task".to_string(),
                field: "deadline".to_string(),
                message: "cannot be in the past.".to_string(),
            })
        } else {
            Ok(())
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        todo!()
    }

    /// Begin a query to retrieve the task's associated `Project`
    pub fn project(&self) -> Find<projects::table, Uuid> {
        projects::table.find(self.project_id)
    }

    /// Begin a query to retrieve the task's assigned `User`s
    pub fn assignees(&self) -> RelatedUsers {
        tasks_users::table
            .filter(tasks_users::task_id.eq(self.id))
            .inner_join(users::table)
            .select(User::as_select())
    }
}
