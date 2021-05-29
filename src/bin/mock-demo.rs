pub mod mock;

use anyhow::Result;
use mock::dao::MockNewTodo;
use mock::service;
use todo::TodoService;

#[async_std::main]
async fn main() -> Result<()> {
    let mut svc = service::MockService::new();

    let pid = svc
        .write_new(MockNewTodo {
            memo: "Hello".to_string(),
        })
        .await?
        .expect("Boon!");

    println!("{:#?}", pid);

    let pid = svc
        .write_new(MockNewTodo {
            memo: "World".to_string(),
        })
        .await?
        .expect("Boon!");

    println!("{:#?}", pid);

    if let Some(pid) = svc.get_todo_by_id(2).await? {
        println!("Found: {:#?}", pid);
    }

    svc.publish(1).await?;

    println!("DRAFT");
    for p in svc.list_draft().await?.into_iter() {
        println!("{:#?}", p);
    }

    println!("PUBLISHED");
    for p in svc.list_published().await?.into_iter() {
        println!("{:#?}", p);
    }

    Ok(())
}
