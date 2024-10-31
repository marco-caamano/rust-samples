use std::thread;
use std::time::Duration;
use tokio::time::sleep;

// tokio defaults to using a thread pool to execute them in parallel
// we can force tokio to use a single thread and do time slicing by
// using (flavor="current_thread")

#[tokio::main]
//#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
    let mut handles = vec![];

    let handle = tokio::spawn(async move {
        foo().await;
    });
    handles.push(handle);

    let handle = tokio::spawn(async move {
        bar().await;
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn foo() {
    println!("[foo|{:?}] Start Execution", thread::current().id());
    for id in 1..10 {
        println!(
            "[foo|{:?}] ID[{}] reading from DB",
            thread::current().id(),
            id
        );
        let value = read_from_db(id).await;
        println!("[foo|{:?}] Got value: {}\n", thread::current().id(), value);
    }
}

async fn bar() {
    println!("[bar|{:?}] Start Execution", thread::current().id());
    for id in 10..20 {
        println!(
            "[bar|{:?}] ID[{}] reading from DB",
            thread::current().id(),
            id
        );
        let value = read_from_db(id).await;
        println!("[bar|{:?}] Got value: {}\n", thread::current().id(), value);
    }
}

async fn read_from_db(id: u32) -> String {
    sleep(Duration::from_millis(50)).await;
    println!("[read_from_db] called from {:?}", thread::current().id());
    format!("Hello there #{}", id)
}
