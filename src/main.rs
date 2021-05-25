extern crate todo;

pub mod mock {
    pub mod dao {

        use todo::post::*;

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

        impl PostDao for MockDao {
            type NewPost = MockNewPost;
            type Post = MockPost;
            type PostId = u32;

            fn create(&mut self, post: MockNewPost) -> Option<u32> {
                self.id_counter = self.id_counter + 1;
                self.drafts.push(MockPost {
                    id: self.id_counter,
                    memo: post.memo,
                });
                println!("CREATED: {:#?}", self);
                Some(self.id_counter)
            }
            fn list_draft(&self) -> Vec<MockPost> {
                self.drafts.clone()
            }
            fn list_published(&self) -> Vec<MockPost> {
                self.published.clone()
            }
            fn get_by_id(&self, id: Self::PostId) -> Option<MockPost> {
                self.drafts.clone().into_iter().find(|p| p.id == id).or(self
                    .published
                    .clone()
                    .into_iter()
                    .find(|p| p.id == id))
            }
            fn publish(&mut self, id: u32) -> bool {
                for (i, v) in self.drafts.clone().into_iter().enumerate() {
                    if v.id == id {
                        self.drafts.remove(i);
                        self.published.push(v);
                        return true;
                    }
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
    }

    pub mod service {
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
        impl HavePostService for MockService {
            type PostService = Self;
            fn post_service(&mut self) -> &mut Self {
                self
            }
        }
    }
}

use mock::service::*;

fn main() {
    let mut svc = MockService::new();
    println!("{:#?}", svc);

    let pid = svc.write_new(MockNewPost {
        memo: "Hello".to_string(),
    });
    println!("{:#?}", pid);

    let pid = svc.write_new(MockNewPost {
        memo: "World".to_string(),
    });
    println!("{:#?}", pid);

    println!("{:#?}", svc);
}
