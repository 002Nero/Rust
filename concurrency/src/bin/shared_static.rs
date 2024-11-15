#[tokio::main]
async fn main() {
    // Attend un entier n en ligne de commande, puis affiche "bonjour numero i"
    // et "au revoir numero i" sur 2 lignes, où i varie de 0 à n-1.
    // Tout se fait en n tâches. La tâche principale attend que toutes les tâches secondaires soient terminées.

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} n", args[0]);
        std::process::exit(1);
    }

    let n: u32 = args[1].parse().expect("n n'est pas un nombre");
    let greeting: &str = "bonjour"; // Chaîne partagée entre les threads

    let mut handles = vec![];
    for i in 0..n {
        // Clonage de la référence `greeting` pour la partager entre threads
        let greeting_ref = greeting;
        let handle = std::thread::spawn(move || {
            println!("{} numero {}", greeting_ref, i);
            println!("au revoir numero {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
