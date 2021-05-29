use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TodoDao {
    type NewTodo: Send;
    type Todo: Send;
    type TodoId: Copy + Send;

    async fn create(&mut self, todo: Self::NewTodo) -> Result<Option<Self::TodoId>>;
    async fn list_draft(&self) -> Result<Vec<Self::Todo>>;
    async fn list_published(&self) -> Result<Vec<Self::Todo>>;
    async fn get_by_id(&self, id: Self::TodoId) -> Result<Option<Self::Todo>>;
    async fn publish(&mut self, id: Self::TodoId) -> Result<bool>;
}
pub trait HaveTodoDao {
    type TodoDao: TodoDao + Sync + Send;
    fn todo_dao(&mut self) -> &mut Self::TodoDao;
}
#[async_trait]
pub trait TodoService: HaveTodoDao {
    async fn write_new(
        &mut self,
        todo: <<Self as HaveTodoDao>::TodoDao as TodoDao>::NewTodo,
    ) -> Result<Option<<<Self as HaveTodoDao>::TodoDao as TodoDao>::TodoId>> {
        let v = self.todo_dao().create(todo).await?;
        Ok(v)
    }

    async fn list_draft(
        &mut self,
    ) -> Result<Vec<<<Self as HaveTodoDao>::TodoDao as TodoDao>::Todo>> {
        let v = self.todo_dao().list_draft().await?;
        Ok(v)
    }

    async fn list_published(
        &mut self,
    ) -> Result<Vec<<<Self as HaveTodoDao>::TodoDao as TodoDao>::Todo>> {
        let v = self.todo_dao().list_published().await?;
        Ok(v)
    }

    async fn get_todo_by_id(
        &mut self,
        id: <<Self as HaveTodoDao>::TodoDao as TodoDao>::TodoId,
    ) -> Result<Option<<<Self as HaveTodoDao>::TodoDao as TodoDao>::Todo>> {
        let v = self.todo_dao().get_by_id(id).await?;
        Ok(v)
    }

    async fn publish(
        &mut self,
        id: <<Self as HaveTodoDao>::TodoDao as TodoDao>::TodoId,
    ) -> Result<bool> {
        let v = self.todo_dao().publish(id).await?;
        Ok(v)
    }
}

impl<T: HaveTodoDao> TodoService for T {}

pub trait HaveTodoService {
    type TodoService: TodoService;
    fn todo_service(&mut self) -> &mut Self::TodoService;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
