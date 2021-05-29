use anyhow::Result;
use async_trait::async_trait;
use todo::TodoDao;

#[derive(Debug)]
pub struct MockNewTodo {
    pub memo: String,
}

#[derive(Debug, Clone)]
pub struct MockTodo {
    pub id: u32,
    pub memo: String,
}

#[derive(Debug)]
pub struct MockDao {
    id_counter: u32,
    drafts: Vec<MockTodo>,
    published: Vec<MockTodo>,
}

impl MockDao {
    pub fn new() -> Self {
        MockDao {
            id_counter: 0,
            drafts: vec![],
            published: vec![],
        }
    }
}

#[async_trait]
impl TodoDao for MockDao {
    type NewTodo = MockNewTodo;
    type Todo = MockTodo;
    type TodoId = u32;

    async fn create(&mut self, todo: MockNewTodo) -> Result<Option<u32>> {
        self.id_counter += 1;
        self.drafts.push(MockTodo {
            id: self.id_counter,
            memo: todo.memo,
        });
        Ok(Some(self.id_counter))
    }
    async fn list_draft(&self) -> Result<Vec<MockTodo>> {
        Ok(self.drafts.clone())
    }
    async fn list_published(&self) -> Result<Vec<MockTodo>> {
        Ok(self.published.clone())
    }
    async fn get_by_id(&self, id: Self::TodoId) -> Result<Option<MockTodo>> {
        for v in self.drafts.iter() {
            if v.id == id {
                return Ok(Some(v.clone()));
            }
        }

        Ok(None)
    }
    async fn publish(&mut self, id: u32) -> Result<bool> {
        if let Ok(i) = self.drafts.binary_search_by(|p| p.id.cmp(&id)) {
            let v = self.drafts.remove(i);
            self.published.push(v);
            return Ok(true);
        }
        Ok(false)
    }
}
