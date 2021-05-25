use futures::TryStreamExt; // try_next()
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::*;

#[derive(Debug, sqlx::FromRow)]
struct Todos {
    id: i32,
    title: String,
    body: Option<String>,
    published: bool,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:admin@localhost:15432/sampledb")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&conn)
        .await?;
    println!("SELECT: {}", row.0);

    let mut rows = sqlx::query("SELECT * FROM Todos").fetch(&conn);
    while let Some(row) = rows.try_next().await? {
        let title: &str = row.try_get("title")?;
        println!("TITLE: {}", title);
    }

    Ok(())
}
