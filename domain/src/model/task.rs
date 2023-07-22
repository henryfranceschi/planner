use chrono::{DateTime, Utc};
use diesel::helper_types::{AsSelect, Find, FindBy, InnerJoin, Select};
use diesel::prelude::*;
use uuid::Uuid;

use crate::model::user::User;
use crate::schema::{projects, tasks, tasks_users, users};

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: Uuid,
    pub name: String,
    pub description: Option<String>,
    deadline: Option<DateTime<Utc>>,
    done: bool,
    project_id: Uuid,
}

type RelatedUsers = Select<
    InnerJoin<FindBy<tasks_users::table, tasks_users::task_id, Uuid>, users::table>,
    AsSelect<User, diesel::pg::Pg>,
>;

impl Task {
    fn new(
        name: String,
        description: Option<String>,
        deadline: Option<DateTime<Utc>>,
        project_id: Uuid,
    ) -> anyhow::Result<Self> {

        if let Some(deadline) = deadline {
            Self::validate_deadline(&deadline)?;
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            deadline,
            done: false,
            project_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn deadline(&self) -> Option<DateTime<Utc>> {
        self.deadline
    }

    fn validate_deadline(deadline: &DateTime<Utc>) -> anyhow::Result<()> {
        if deadline < &Utc::now() {
            anyhow::bail!("deadline cannot be in the past.");
        } else {
            Ok(())
        }
    }

    pub fn set_deadline(&mut self, deadline: DateTime<Utc>) -> anyhow::Result<()> {
        Self::validate_deadline(&deadline)?;

        self.deadline = Some(deadline);

        Ok(())
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
