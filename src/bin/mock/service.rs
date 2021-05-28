pub use super::dao::*;
pub use todo::post::*;

#[derive(Debug)]
pub struct MockService {
    post_dao: MockDao,
}

impl MockService {
    pub fn new() -> Self {
        MockService {
            post_dao: MockDao::new(),
        }
    }
}
impl HavePostDao for MockService {
    type PostDao = MockDao;
    fn post_dao(&mut self) -> &mut MockDao {
        &mut self.post_dao
    }
}
