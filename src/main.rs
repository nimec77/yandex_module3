use std::future::Future;
use std::time::{Duration, Instant};

use futures::future::join_all;
use tokio::time::sleep;

async fn handle_connections<I, F>(connections: I)
where
    I: IntoIterator<Item = F>,
    F: Future<Output: Send + 'static> + Send + 'static,
{
    let mut handles = vec![];
    for connection in connections {
        handles.push(tokio::spawn(connection));
    }
    join_all(handles).await;
}

#[tokio::main]
async fn main() {
    let connections = {
        let mut connections = Vec::with_capacity(10);
        for i in 0..10 {
            let connection = async move {
                sleep(Duration::from_millis(10)).await;
                println!("Connection {} handled", i);
            };
            connections.push(connection);
        }
        connections
    };

    let start = Instant::now();

    handle_connections(connections).await;

    let end = start.elapsed();

    assert!(end < Duration::from_millis(500))
}
