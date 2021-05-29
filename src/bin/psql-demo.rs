pub mod psql;

use anyhow::Result;
use psql::dao::NewTodos;
use psql::service;
use todo::TodoService;

#[async_std::main]
async fn main() -> Result<()> {
    let mut svc = service::TodoService::new(String::from(
        "postgres://admin:admin@localhost:15432/sampledb",
    ))
    .await?;

    let pid = svc
        .write_new(NewTodos {
            title: "Hello".to_string(),
            body: Some(
                "Press this button, and it will be ready for your coffee in no time.".to_string(),
            ),
        })
        .await?
        .expect("Boon!");

    println!("{}", pid);

    let pid = svc
        .write_new(NewTodos {
            title: "World".to_string(),
            body: Some("We'd better hurry!".to_string()),
        })
        .await?
        .expect("Boon!");

    println!("{:?}", pid);

    if let Some(p) = svc.get_todo_by_id(pid).await? {
        println!("Found: {:?}", p);
    }

    svc.publish(pid).await?;

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
