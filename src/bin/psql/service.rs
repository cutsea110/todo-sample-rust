use anyhow::Result;

use super::dao::PgTodoDao;
use todo::HaveTodoDao;

#[derive(Debug)]
pub struct TodoService {
    todo_dao: PgTodoDao,
}

impl TodoService {
    pub async fn new(conn_str: String) -> Result<Self> {
        let conn = PgTodoDao::new(conn_str, 5).await?;
        Ok(TodoService { todo_dao: conn })
    }
}

impl HaveTodoDao for TodoService {
    type TodoDao = PgTodoDao;
    fn todo_dao(&mut self) -> &mut PgTodoDao {
        &mut self.todo_dao
    }
}
