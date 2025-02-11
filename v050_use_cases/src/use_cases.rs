use serde::Deserialize;
use crate::domain::{BallotPaper, VoteOutcome};
use crate::storages::file::VotingMachine;

#[derive(Deserialize)]
pub struct VoteForm{
    pub voter : String,
    pub Candidate : String,
}

impl From<VoteForm> for BallotPaper {
    fn from(form: VoteForm) -> Self {
        //TODO
    }
}

pub struct VotingController<Store>{
    store: Store,
}

impl<Store> VotingController<Store>{
    pub fn new(store: Store) -> Self
    
    pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<(VoteOutcome)>
    
    pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>
}

