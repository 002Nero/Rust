use std::sync::{Arc, Mutex};
use std::thread;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} n", args[0]);
        std::process::exit(1);
    }

    let n: u32 = args[1].parse().expect("n est pas un nombre");
    let counter = Arc::new(Mutex::new(0)); // Partage du compteur entre les threads
    let mut handles = vec![];

    for _ in 0..n {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("bonjour numero {}", *num);
            println!("au revoir numero {}", *num);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
