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
    let total = Arc::new(Mutex::new(0)); // Partage de l'entier mutable entre les threads
    let mut handles = vec![];

    for _ in 0..n {
        let total = Arc::clone(&total);
        let handle = thread::spawn(move || {
            let mut num = total.lock().unwrap(); // Verrouille l'accès à "total"
            *num += 1;
            println!("La valeur actuelle de total est : {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Assure que toutes les tâches sont terminées
    }

    let final_total = *total.lock().unwrap(); // Lecture finale de "total"
    println!("Valeur finale de total : {}", final_total);
}
