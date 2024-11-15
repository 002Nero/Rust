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

    // La chaîne allouée dynamiquement
    let greeting = String::from("bonjour");

    // Utilisation d'Arc pour partager la chaîne entre les threads
    let greeting_ref = std::sync::Arc::new(greeting);

    let mut handles = vec![];
    for i in 0..n {
        // Clone l'Arc pour chaque thread
        let greeting_ref = std::sync::Arc::clone(&greeting_ref);
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

//Chaîne String:
// 
// Segment de mémoire: La chaîne elle-même est allouée sur le tas (heap). Le pointeur, la longueur et la capacité du String sont stockés sur la pile (stack) dans l'objet String.
// Durée de vie: La chaîne reste vivante tant qu'il existe des références actives via Arc.
// Arc:
// 
// Segment de mémoire: L'objet Arc est stocké sur le tas et contient un compteur atomique pour gérer le nombre de références.
// Durée de vie: Automatiquement gérée par le compteur de référence de l'Arc.