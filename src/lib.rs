pub mod post {
    pub trait PostDao {
        type NewPost;
        type Post;
        type PostId;

        fn create(&mut self, post: Self::NewPost) -> Option<Self::PostId>;
        fn list_draft(&self) -> Vec<Self::Post>;
        fn list_published(&self) -> Vec<Self::Post>;
        fn get_by_id(&self, id: Self::PostId) -> Option<Self::Post>;
        fn publish(&mut self, id: Self::PostId) -> bool;
    }
    pub trait HavePostDao {
        type PostDao: PostDao;
        fn post_dao(&self) -> Self::PostDao;
    }
    pub trait PostService: HavePostDao {
        fn write_new(
            &self,
            post: <<Self as HavePostDao>::PostDao as PostDao>::NewPost,
        ) -> Option<<<Self as HavePostDao>::PostDao as PostDao>::PostId> {
            self.post_dao().create(post)
        }

        fn list_draft(&self) -> Vec<<<Self as HavePostDao>::PostDao as PostDao>::Post> {
            self.post_dao().list_draft()
        }

        fn list_published(&self) -> Vec<<<Self as HavePostDao>::PostDao as PostDao>::Post> {
            self.post_dao().list_published()
        }

        fn get_post_by_id(
            &self,
            id: <<Self as HavePostDao>::PostDao as PostDao>::PostId,
        ) -> Option<<<Self as HavePostDao>::PostDao as PostDao>::Post> {
            self.post_dao().get_by_id(id)
        }

        fn publish(&self, id: <<Self as HavePostDao>::PostDao as PostDao>::PostId) -> bool {
            self.post_dao().publish(id)
        }
    }

    impl<T: HavePostDao> PostService for T {}

    pub trait HavePostService {
        type PostService: PostService;
        fn post_service(&self) -> Self::PostService;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
