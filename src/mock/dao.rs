use todo::post::*;

#[derive(Debug)]
pub struct MockNewPost {
    pub memo: String,
}
#[derive(Debug)]
pub struct MockPost {
    pub id: u32,
    pub memo: String,
}

#[derive(Debug)]
pub struct MockDao {
    id_counter: u32,
    drafts: Vec<MockPost>,
    published: Vec<MockPost>,
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

impl PostDao for MockDao {
    type NewPost = MockNewPost;
    type Post = MockPost;
    type PostId = u32;

    fn create(&mut self, post: MockNewPost) -> Option<u32> {
        self.id_counter += 1;
        self.drafts.push(MockPost {
            id: self.id_counter,
            memo: post.memo,
        });
        Some(self.id_counter)
    }
    fn list_draft(&self) -> &[MockPost] {
        self.drafts.as_slice()
    }
    fn list_published(&self) -> &[MockPost] {
        self.published.as_slice()
    }
    fn get_by_id(&self, id: Self::PostId) -> Option<&MockPost> {
        self.drafts
            .as_slice()
            .into_iter()
            .find(|p| p.id == id)
            .or(self.published.as_slice().into_iter().find(|p| p.id == id))
    }
    fn publish(&mut self, id: u32) -> bool {
        if let Ok(i) = self.drafts.binary_search_by(|p| p.id.cmp(&id)) {
            let v = self.drafts.remove(i);
            self.published.push(v);
            return true;
        }
        false
    }
}
impl HavePostDao for MockDao {
    type PostDao = MockDao;
    fn post_dao(&mut self) -> &mut MockDao {
        self
    }
}
