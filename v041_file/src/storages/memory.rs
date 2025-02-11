use async_trait::async_trait;
use anyhow::Result;
use crate::domain::VotingMachine;
use crate::storage::Storage;
pub(crate) struct MemoryStore {
    voting_machine: VotingMachine,
}



#[async_trait]
impl Storage for MemoryStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        Ok(MemoryStore { voting_machine: machine })
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        Ok(self.voting_machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        self.voting_machine = machine;
        Ok(())
    }
}