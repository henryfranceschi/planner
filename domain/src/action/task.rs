use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    error::DomainError,
    model::{project::Project, task::Task},
    schema::tasks,
};

use super::Context;

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct CreateTask {
    name: String,
    description: Option<String>,
    deadline: Option<DateTime<Utc>>,
    done: bool,
}

pub async fn create(
    ctx: &mut Context<'_>,
    project: &Project,
    args: CreateTask,
) -> Result<Task, DomainError> {
    // if project::is_member(ctx, project, ctx.user()?).await? {
    //     return Err(DomainError::Authorization);
    // }

    let task = Task::new(
        args.name,
        args.description,
        args.deadline,
        args.done,
        project.id(),
    )?;

    diesel::insert_into(tasks::table)
        .values(&task)
        .execute(ctx.conn)
        .await?;

    Ok(task)
}

pub async fn find(ctx: &mut Context<'_>, id: Uuid) -> Result<Task, DomainError> {
    let task = tasks::table
        .find(id)
        .select(Task::as_select())
        .get_result(ctx.conn)
        .await?;

    Ok(task)
}

pub async fn delete(ctx: &mut Context<'_>, id: Uuid) -> Result<(), DomainError> {
    diesel::delete(tasks::table.filter(tasks::id.eq(id)))
        .execute(ctx.conn)
        .await?;

    Ok(())
}
