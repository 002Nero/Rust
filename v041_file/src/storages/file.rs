use async_trait::async_trait;
use anyhow::Result;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct FileStore {
    filepath: String,
}

const FILEPATH: &str = "machine.json";
impl FileStore {
    pub async fn create(machine: VotingMachine, filepath: &str) -> Result<Self> {
        let mut store = FileStore {
            filepath: filepath.to_string(),
        };
        store.put_voting_machine(machine).await?;
        Ok(store)
    }
}

#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        FileStore::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let data = fs::read(&self.filepath).await?;
        let machine: VotingMachine = serde_json::from_slice(&data)?;
        Ok(machine)
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        let data = serde_json::to_vec(&machine)?;
        let mut file = fs::File::create(&self.filepath).await?;
        file.write_all(&data).await?;
        Ok(())
    }
}