use diesel::{dsl::exists, prelude::*};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    error::DomainError,
    model::{
        project::{Project, ProjectUser},
        user::User,
    },
    schema::{projects, projects_users, users},
};

use super::Context;

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct CreateProject {
    name: String,
    description: Option<String>,
    archived: bool,
    client_id: Option<Uuid>,
}

pub async fn create(ctx: &mut Context<'_>, args: CreateProject) -> Result<Project, DomainError> {
    let project = Project::new(args.name, args.description, args.archived, args.client_id);

    diesel::insert_into(projects::table)
        .values(&project)
        .execute(ctx.conn)
        .await?;

    Ok(project)
}

pub async fn find(ctx: &mut Context<'_>, id: Uuid) -> Result<Project, DomainError> {
    Ok(projects::table
        .select(Project::as_select())
        .find(id)
        .first(ctx.conn)
        .await?)
}

pub async fn delete(ctx: &mut Context<'_>, id: Uuid) -> Result<(), DomainError> {
    diesel::delete(projects::table.find(id))
        .execute(ctx.conn)
        .await?;

    Ok(())
}

pub async fn create_member(
    ctx: &mut Context<'_>,
    project: &Project,
    user: &User,
) -> Result<(), DomainError> {
    diesel::insert_into(projects_users::table)
        .values((
            projects_users::project_id.eq(project.id()),
            projects_users::user_id.eq(user.id()),
        ))
        .execute(ctx.conn)
        .await?;

    Ok(())
}

pub async fn members(ctx: &mut Context<'_>, project: &Project) -> Result<Vec<User>, DomainError> {
    let members: Vec<User> = ProjectUser::belonging_to(project)
        .inner_join(users::table)
        .select(User::as_select())
        .load(ctx.conn)
        .await?;

    Ok(members)
}

pub async fn delete_member(
    ctx: &mut Context<'_>,
    project: &Project,
    user: &User,
) -> Result<(), DomainError> {
    diesel::delete(
        projects_users::table.filter(
            projects_users::project_id
                .eq(project.id())
                .and(projects_users::user_id.eq(user.id())),
        ),
    )
    .execute(ctx.conn)
    .await?;

    Ok(())
}

pub async fn is_member(
    ctx: &mut Context<'_>,
    project: &Project,
    user: &User,
) -> Result<bool, DomainError> {
    let member_exists = diesel::select(exists(
        projects_users::table.filter(
            projects_users::user_id
                .eq(user.id())
                .and(projects_users::project_id.eq(project.id())),
        ),
    ))
    .get_result(ctx.conn)
    .await?;

    Ok(member_exists)
}
