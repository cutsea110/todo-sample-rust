use super::dao::MockDao;
use todo::HaveTodoDao;

#[derive(Debug)]
pub struct MockService {
    todo_dao: MockDao,
}

impl MockService {
    pub fn new() -> Self {
        MockService {
            todo_dao: MockDao::new(),
        }
    }
}
impl HaveTodoDao for MockService {
    type TodoDao = MockDao;
    fn todo_dao(&mut self) -> &mut MockDao {
        &mut self.todo_dao
    }
}
