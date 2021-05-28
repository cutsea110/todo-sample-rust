use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PostDao {
    type NewPost: Send;
    type Post: Send;
    type PostId: Copy + Send;

    async fn create(&mut self, post: Self::NewPost) -> Result<Option<Self::PostId>>;
    async fn list_draft(&self) -> Result<Vec<Self::Post>>;
    async fn list_published(&self) -> Result<Vec<Self::Post>>;
    async fn get_by_id(&self, id: Self::PostId) -> Result<Option<Self::Post>>;
    async fn publish(&mut self, id: Self::PostId) -> Result<bool>;
}
pub trait HavePostDao {
    type PostDao: PostDao + Sync + Send;
    fn post_dao(&mut self) -> &mut Self::PostDao;
}
#[async_trait]
pub trait PostService: HavePostDao {
    async fn write_new(
        &mut self,
        post: <<Self as HavePostDao>::PostDao as PostDao>::NewPost,
    ) -> Result<Option<<<Self as HavePostDao>::PostDao as PostDao>::PostId>> {
        let v = self.post_dao().create(post).await?;
        Ok(v)
    }

    async fn list_draft(
        &mut self,
    ) -> Result<Vec<<<Self as HavePostDao>::PostDao as PostDao>::Post>> {
        let v = self.post_dao().list_draft().await?;
        Ok(v)
    }

    async fn list_published(
        &mut self,
    ) -> Result<Vec<<<Self as HavePostDao>::PostDao as PostDao>::Post>> {
        let v = self.post_dao().list_published().await?;
        Ok(v)
    }

    async fn get_post_by_id(
        &mut self,
        id: <<Self as HavePostDao>::PostDao as PostDao>::PostId,
    ) -> Result<Option<<<Self as HavePostDao>::PostDao as PostDao>::Post>> {
        let v = self.post_dao().get_by_id(id).await?;
        Ok(v)
    }

    async fn publish(
        &mut self,
        id: <<Self as HavePostDao>::PostDao as PostDao>::PostId,
    ) -> Result<bool> {
        let v = self.post_dao().publish(id).await?;
        Ok(v)
    }
}

impl<T: HavePostDao> PostService for T {}

pub trait HavePostService {
    type PostService: PostService;
    fn post_service(&mut self) -> &mut Self::PostService;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
