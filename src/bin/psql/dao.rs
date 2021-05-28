use anyhow::Result;
use async_trait::async_trait;
use futures::TryStreamExt; // try_next()
use sqlx::postgres::{PgPool, PgPoolOptions};

use todo::*;

#[derive(Debug)]
pub struct NewTodos {
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Todos {
    id: i32,
    title: String,
    body: Option<String>,
    published: bool,
}

#[derive(Debug)]
pub struct PgTodosDao {
    conn: PgPool,
}

impl PgTodosDao {
    pub async fn new(conn_str: String, conn_max: u32) -> Result<Self> {
        let conn = PgPoolOptions::new()
            .max_connections(conn_max)
            .connect(&conn_str[..])
            .await?;

        Ok(PgTodosDao { conn })
    }
}

#[async_trait]
impl PostDao for PgTodosDao {
    type NewPost = NewTodos;
    type Post = Todos;
    type PostId = i32;

    async fn create(&mut self, post: NewTodos) -> Result<Option<i32>> {
        let row: (i32,) =
            sqlx::query_as("INSERT INTO todos (title, body) VALUES ($1, $2) RETURNING id")
                .bind(post.title)
                .bind(post.body)
                .fetch_one(&self.conn)
                .await?;

        Ok(Some(row.0))
    }
    async fn list_draft(&self) -> Result<Vec<Todos>> {
        let mut rows = sqlx::query_as::<_, Todos>(
            "SELECT id, title, body, published FROM todos WHERE published = 'f'",
        )
        .fetch(&self.conn);

        let mut v = vec![];
        while let Some(row) = rows.try_next().await? {
            v.push(row);
        }
        Ok(v)
    }
    async fn list_published(&self) -> Result<Vec<Todos>> {
        let mut rows = sqlx::query_as::<_, Todos>(
            "SELECT id, title, body, published FROM todos WHERE published = 't'",
        )
        .fetch(&self.conn);

        let mut v = vec![];
        while let Some(row) = rows.try_next().await? {
            v.push(row);
        }
        Ok(v)
    }
    async fn get_by_id(&self, id: i32) -> Result<Option<Todos>> {
        let row = sqlx::query_as::<_, Todos>(
            "SELECT id, title, body, published FROM todos WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.conn)
        .await?;

        Ok(Some(row))
    }
    async fn publish(&mut self, id: i32) -> Result<bool> {
        sqlx::query("UPDATE todos SET published = 't' WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(true)
    }
}
