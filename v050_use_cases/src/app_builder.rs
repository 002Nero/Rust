use crate::configuration::Configuration;
use crate::domain::{VotingMachine, Voter, BallotPaper, VoteOutcome};
use crate::storages::memory::MemoryStore;
use crate::storage::Storage;
use anyhow::Result;
use std::io::{self, Write};

pub async fn run_app(configuration: Configuration) -> Result<()> {
    let voting_machine = VotingMachine::new();
    let mut storage = MemoryStore::new(voting_machine).await?;

    println!("Candidates: {:?}", configuration.candidates);

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

                let voter = Voter(name.clone());

                println!("Veuillez choisir un candidat parmi les suivants:");
                for (i, candidate) in configuration.candidates.iter().enumerate() {
                    println!("{}. {}", i + 1, candidate);
                }

                let mut candidat = String::new();
                io::stdin().read_line(&mut candidat).expect("Erreur de lecture");

                let candidate = candidat.trim().parse::<usize>().ok().and_then(|index| {
                    if index > 0 && index <= configuration.candidates.len() {
                        Some(configuration.candidates[index - 1].clone())
                    } else {
                        None
                    }
                });

                let ballot = BallotPaper {
                    voter: voter.clone(),
                    candidate: candidate.map(|c| c.into()),
                };

                let mut voting_machine = storage.get_voting_machine().await?;
                match voting_machine.vote(ballot) {
                    VoteOutcome::AcceptedVote(_, candidate) => {
                        println!("Vous avez voté pour le candidat {}", candidate.0);
                    }
                    VoteOutcome::BlankVote(_) => {
                        println!("Vous avez voté blanc.");
                    }
                    VoteOutcome::InvalidVote(_) => {
                        println!("Vote nul.");
                    }
                    VoteOutcome::HasAlreadyVoted(_) => {
                        println!("Vous avez déjà voté.");
                    }
                }
                storage.put_voting_machine(voting_machine).await?;
            }
            Ok(2) => {
                let voting_machine = storage.get_voting_machine().await?;
                println!("Liste des votants:");
                for voter in &voting_machine.get_voters().0 {
                    println!("{}", voter.0);
                }
            }
            Ok(3) => {
                let voting_machine = storage.get_voting_machine().await?;
                println!("Scores des candidats:");
                for (candidate, score) in &voting_machine.get_scoreboard().scores {
                    println!("Candidat {}: {} votes", candidate.0, score.0);
                }
                println!("Votes blancs: {}", voting_machine.get_scoreboard().blank_score.0);
                println!("Votes nuls: {}", voting_machine.get_scoreboard().invalid_score.0);
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

    Ok(())
}