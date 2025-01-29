use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

pub struct Voter(pub String);

pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendenceSheet(pub Set<Voter>);

pub struct Scoreboard{
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

pub struct BallotPaper{
    pub voter: Voter,
    pub candidate: Option<Candidate>
}

pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine{
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard {
            scores: Map::new(),
            blank_score: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl VotingMachine {
    pub fn new() -> Self {
        VotingMachine {
            voters: AttendenceSheet(Set::new()),
            scoreboard: Scoreboard::new(),
        }
    }
}

impl VotingMachine {
    pub fn vote(&mut self, ballot: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot.voter);
        }

        self.voters.0.insert(ballot.voter.clone());

        match ballot.candidate {
            Some(candidate) => {
                let score = self.scoreboard.scores.entry(candidate.clone()).or_insert(Score(0));
                score.0 += 1;
                VoteOutcome::AcceptedVote(ballot.voter, candidate)
            }
            None => {
                self.scoreboard.blank_score.0 += 1;
                VoteOutcome::BlankVote(ballot.voter)
            }
        }
    }
}

impl VotingMachine{
    pub fn get_scoreboard(&self) -> &Scoreboard{
        &self.scoreboard

    }
    pub fn get_voters(&self) -> &AttendenceSheet{
        &self.voters
    }
}

fn create_voting_machine() -> VotingMachine {
    VotingMachine::new()
}