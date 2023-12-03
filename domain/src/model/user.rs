use std::sync::Arc;

use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHashString, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::{
    backend::Backend, deserialize::FromSql, serialize::ToSql, AsExpression, FromSqlRow, SqlType,
};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Insertable, Selectable, Identifiable)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    id: Uuid,
    pub email_address: String,
    password_hash: PasswordHash,
}

impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn validate_password(password: &str) -> anyhow::Result<()> {
        let _ = password;

        todo!()
    }

    pub async fn set_password(&mut self, password: String) -> anyhow::Result<()> {
        Self::validate_password(&password)?;

        self.password_hash = PasswordHash::new(password).await?;

        Ok(())
    }

    pub async fn verify_password(&mut self, password: String) -> anyhow::Result<()> {
        self.password_hash.verify_password(password).await
    }
}

#[derive(AsExpression, FromSqlRow, SqlType, Debug, Clone)]
#[diesel(sql_type = diesel::sql_types::Text)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PasswordHash(Arc<PasswordHashString>);

impl PasswordHash {
    pub async fn new(password: String) -> anyhow::Result<Self> {
        let password_hash =
            tokio::task::spawn_blocking(move || -> anyhow::Result<PasswordHashString> {
                let salt = SaltString::generate(&mut OsRng);
                let password_hasher = Argon2::default();

                password_hasher
                    .hash_password(password.as_ref(), &salt)
                    .map_err(|e| anyhow!("failed to hash password: {e}"))
                    .map(Into::into)
            })
            .await
            .map_err(|e| anyhow!("thread panicked while hashing password: {e}"))??;

        Ok(Self(Arc::new(password_hash)))
    }

    pub async fn verify_password(&self, password: String) -> anyhow::Result<()> {
        let password_hash = self.0.clone();

        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            Argon2::default()
                .verify_password(password.as_ref(), &password_hash.password_hash())
                .map_err(|e| anyhow!("failed to verify password: {e}"))
        })
        .await
        .map_err(|e| anyhow!("thread panicked while verifying password: {e}"))??;

        Ok(())
    }
}

impl<DB> ToSql<diesel::sql_types::Text, DB> for PasswordHash
where
    str: ToSql<diesel::sql_types::Text, DB>,
    DB: Backend,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0.as_str().to_sql(out)
    }
}

impl<DB> FromSql<diesel::sql_types::Text, DB> for PasswordHash
where
    String: FromSql<diesel::sql_types::Text, DB>,
    DB: Backend,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let string = String::from_sql(bytes)?;

        Ok(PasswordHash(Arc::new(PasswordHashString::new(&string)?)))
    }
}
