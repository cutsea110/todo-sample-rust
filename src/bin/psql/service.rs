use anyhow::Result;

pub use super::dao::*;
pub use todo::*;

#[derive(Debug)]
pub struct TodoService {
    post_dao: PgTodoDao,
}

impl TodoService {
    pub async fn new(conn_str: String) -> Result<Self> {
        let conn = PgTodoDao::new(conn_str, 5).await?;
        Ok(TodoService { post_dao: conn })
    }
}

impl HavePostDao for TodoService {
    type PostDao = PgTodoDao;
    fn post_dao(&mut self) -> &mut PgTodoDao {
        &mut self.post_dao
    }
}
