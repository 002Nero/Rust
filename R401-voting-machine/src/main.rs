
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut voters: Vec<String> = Vec::new();
    let mut scores: HashMap<u32, u32> = HashMap::new();
    let mut blank_votes: u32 = 0;
    let mut null_votes: u32 = 0;

    loop {
        println!("Menu:");
        println!("1. Voter");
        println!("2. Votants");
        println!("3. Scores");
        println!("4. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        if choix.is_empty() {
            println!("Commande vide. Veuillez choisir parmi les options suivantes : 1, 2, 3, 4");
            continue;
        }

        match choix.parse::<u32>() {
            Ok(1) => {
                println!("Veuillez entrer votre nom: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Erreur de lecture");
                let name = name.trim().to_string();

                if name.is_empty() {
                    println!("Nom invalide, veuillez réessayer.");
                    continue;
                }

                if voters.contains(&name) {
                    println!("Vous avez déjà voté.");
                    continue;
                }

                voters.push(name.clone());

                println!("Veuillez choisir un candidat parmi les suivants: \n1. Candidat 1\n2. Candidat 2\n3. Candidat 3");
                let mut candidat = String::new();
                io::stdin().read_line(&mut candidat).expect("Erreur de lecture");

                let candidat = candidat.trim().parse::<u32>();
                match candidat {
                    Ok(1) | Ok(2) | Ok(3) => {
                        *scores.entry(candidat.clone().unwrap()).or_insert(0) += 1;
                        println!("Vous avez voté pour le candidat {}", candidat.unwrap());
                    }
                    Ok(_) => {
                        blank_votes += 1;
                        println!("Vous avez voté blanc.");
                    }
                    Err(_) => {
                        null_votes += 1;
                        println!("Vote nul.");
                    }
                }
            }
            Ok(2) => {
                println!("Liste des votants:");
                for voter in &voters {
                    println!("{}", voter);
                }
            }
            Ok(3) => {
                println!("Scores des candidats:");
                for (candidate, score) in &scores {
                    println!("Candidat {}: {} votes", candidate, score);
                }
                println!("Votes blancs: {}", blank_votes);
                println!("Votes nuls: {}", null_votes);
            }
            Ok(4) => {
                println!("Au revoir!");
                break;
            }
            _ => {
                println!("Choix invalide. Veuillez choisir parmi les options suivantes : 1, 2, 3, 4");
            }
        }
    }
}