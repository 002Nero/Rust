use std::fs::File;
use std::collections::{BTreeSet, HashSet, HashMap};
use std::iter::FromIterator;
use async_trait::async_trait;
use anyhow::Result;
use std::fs::read;
use std::os::windows::fs;
use crate::domain::{Scoreboard, VotingMachine as OtherVotingMachine, AttendenceSheet, Candidate, Score, Voter};
use crate::storage::Storage;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct ScoreboardDao {
    scores: HashMap<String, usize>,
    blank_votes: usize,
    invalid_votes: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VotingMachineDao {
    pub voters: HashSet<String>,
    scoreboard: ScoreboardDao,
}

pub struct VotingMachine {
    pub voters: AttendenceSheet,
    pub scoreboard: Scoreboard,
}

pub struct FileStore {
    filepath: String,
}

const FILEPATH: &str = "machine.json";

impl FileStore {
    pub async fn create(machine: &VotingMachine, filepath: &str) -> Result<Self> {
        let store = FileStore {
            filepath: filepath.to_string(),
        };
        store.put_voting_machine(machine).await?;
        Ok(store)
    }
}

#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        FileStore::create(&machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let data = read(&self.filepath).await?;
        let dao: VotingMachineDao = serde_json::from_slice(&data)?;
        Ok(dao.into())
    }

    async fn put_voting_machine(&self, machine: &VotingMachine) -> Result<()> {
        let data = serde_json::to_vec(&VotingMachineDao::from(machine.clone()))?;
        let mut file = File::create(&self.filepath).await?;
        file.write_all(&data).await?;
        Ok(())
    }
}

impl From<ScoreboardDao> for Scoreboard {
    fn from(dao: ScoreboardDao) -> Self {
        Scoreboard {
            scores: dao.scores.into_iter().map(|(k, v)| (Candidate(k), Score(v))).collect(),
            blank_score: Score(dao.blank_votes),
            invalid_score: Score(dao.invalid_votes),
        }
    }
}

impl From<&VotingMachine> for VotingMachineDao {
    fn from(machine: &VotingMachine) -> Self {
        VotingMachineDao {
            voters: machine.voters.0.iter().map(|v| v.0.clone()).collect(),
            scoreboard: ScoreboardDao {
                scores: machine.scoreboard.scores.iter().map(|(k, v)| (k.0.clone(), v.0)).collect(),
                blank_votes: machine.scoreboard.blank_score.0,
                invalid_votes: machine.scoreboard.invalid_score.0,
            },
        }
    }
}

impl From<VotingMachineDao> for VotingMachine {
    fn from(dao: VotingMachineDao) -> Self {
        VotingMachine {
            voters: AttendenceSheet(dao.voters.into_iter().map(Voter).collect()),
            scoreboard: Scoreboard {
                scores: dao.scoreboard.scores.into_iter().map(|(k, v)| (Candidate(k), Score(v))).collect(),
                blank_score: Score(dao.scoreboard.blank_votes),
                invalid_score: Score(dao.scoreboard.invalid_votes),
            },
        }
    }
}