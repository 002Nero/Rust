fn main() {
    //attends un entier n en ligne de commande puis affiche bonjour numero i puis
    // au revoir numero i sur 2 lignes ou i varie de 0 a n-1
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} n", args[0]);
        std::process::exit(1);
    }
    let n: u32 = args[1].parse().expect("n est pas un nombre");
    for i in 0..n {
        println!("bonjour numero {}", i);
        println!("au revoir numero {}", i);
    }
        
    
    
}