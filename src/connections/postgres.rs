use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .expect("Failed to create pool");

        Ok(Self { pool })
    }

    pub async fn insert_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, sqlx::Error> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, email, password, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users 
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}
