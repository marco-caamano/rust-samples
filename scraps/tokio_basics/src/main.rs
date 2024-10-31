use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // this still runs sequentially...all foo must complete before
    // bar is executed
    foo().await;
    bar().await;
}

async fn foo() {
    println!("[foo] Start Execution");
    for id in 1..10 {
        let value = read_from_db(id).await;
        println!("[{}:{}] Got value: {}", file!(), line!(), value);
    }
}

async fn bar() {
    println!("[bar] Start Execution");
    for id in 10..20 {
        let value = read_from_db(id).await;
        println!("[{}:{}] Got value: {}", file!(), line!(), value);
    }
}

async fn read_from_db(id: u32) -> String {
    thread::sleep(Duration::from_millis(100));
    format!("Hello there #{}", id)
}
