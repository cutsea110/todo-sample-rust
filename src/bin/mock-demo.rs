pub mod mock;
use anyhow::Result;
use mock::service::*;

#[async_std::main]
async fn main() -> Result<()> {
    let mut svc = MockService::new();
    println!("{:#?}", svc);

    let pid = svc
        .write_new(MockNewPost {
            memo: "Hello".to_string(),
        })
        .await?;
    println!("{:#?}", pid);

    let pid = svc
        .write_new(MockNewPost {
            memo: "World".to_string(),
        })
        .await?;
    println!("{:#?}", pid);

    if let Some(pid) = svc.get_post_by_id(2).await? {
        println!("Found: {:#?}", pid);
    }

    svc.publish(1).await?;
    println!("{:#?}", svc);

    for p in svc.list_draft().await?.into_iter() {
        println!("{:#?}", p);
    }
    for p in svc.list_published().await?.into_iter() {
        println!("{:#?}", p);
    }

    Ok(())
}
