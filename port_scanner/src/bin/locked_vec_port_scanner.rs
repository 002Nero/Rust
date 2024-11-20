use clap::Parser;
use port_scanner::{is_open, Parameters};
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() {
    let params = Parameters::parse();
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = vec![];

    for port in params.port_min..=params.port_max {
        let host = params.host.clone();
        let timeout = params.timeout;
        let open_ports = Arc::clone(&open_ports);

        let task = task::spawn(async move {
            if is_open(&host, port, timeout).await {
                let mut open_ports = open_ports.lock().unwrap();
                open_ports.push(port);
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }

    let open_ports = open_ports.lock().unwrap();
    println!("Open ports: {:?}", *open_ports);
}