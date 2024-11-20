use clap::Parser;
use port_scanner::{is_open, Parameters};
use tokio::task;

#[tokio::main]
async fn main() {
    let params = Parameters::parse();

    let mut tasks = vec![];

    for port in params.port_min..=params.port_max {
        let host = params.host.clone();
        let timeout = params.timeout;

        let task = task::spawn(async move {
            if is_open(&host, port, timeout).await {
                println!("Port {} is open", port);
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }
}