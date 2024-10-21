use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.clone().send("hello".to_string()).unwrap();
        println!("hello");
    });
    tx.send("测试".to_string()).unwrap();
    if let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}
