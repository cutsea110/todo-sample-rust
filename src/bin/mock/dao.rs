use anyhow::Result;
use async_trait::async_trait;
use todo::*;

#[derive(Debug)]
pub struct MockNewPost {
    pub memo: String,
}

#[derive(Debug, Clone)]
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

#[async_trait]
impl PostDao for MockDao {
    type NewPost = MockNewPost;
    type Post = MockPost;
    type PostId = u32;

    async fn create(&mut self, post: MockNewPost) -> Result<Option<u32>> {
        self.id_counter += 1;
        self.drafts.push(MockPost {
            id: self.id_counter,
            memo: post.memo,
        });
        Ok(Some(self.id_counter))
    }
    async fn list_draft(&self) -> Result<Vec<MockPost>> {
        Ok(self.drafts.clone())
    }
    async fn list_published(&self) -> Result<Vec<MockPost>> {
        Ok(self.published.clone())
    }
    async fn get_by_id(&self, id: Self::PostId) -> Result<Option<MockPost>> {
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
