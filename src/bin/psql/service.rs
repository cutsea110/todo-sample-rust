use anyhow::Result;

pub use super::dao::*;
pub use todo::post::*;

#[derive(Debug)]
pub struct TodoService {
    post_dao: PgTodosDao,
}

impl TodoService {
    pub async fn new(conn_str: String) -> Result<Self> {
        let conn = PgTodosDao::new(conn_str, 5).await?;
        Ok(TodoService { post_dao: conn })
    }
}

impl HavePostDao for TodoService {
    type PostDao = PgTodosDao;
    fn post_dao(&mut self) -> &mut PgTodosDao {
        &mut self.post_dao
    }
}
