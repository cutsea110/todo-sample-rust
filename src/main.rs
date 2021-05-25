extern crate todo;

pub mod mock;
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

    if let Some(pid) = svc.get_post_by_id(2) {
        println!("Found: {:#?}", pid);
    }

    svc.publish(1);
    println!("{:#?}", svc);

    for p in svc.list_draft().into_iter() {
        println!("{:#?}", p);
    }
    for p in svc.list_published().into_iter() {
        println!("{:#?}", p);
    }
}
