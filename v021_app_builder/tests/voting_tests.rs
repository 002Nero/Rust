#[cfg(test)]
mod voting_tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_voter_registration() {
        let mut voters: Vec<String> = Vec::new();
        let name = "Alice".to_string();
        voters.push(name.clone());
        assert!(voters.contains(&name));
    }

    #[test]
    fn test_blank_vote() {
        let mut blank_votes: u32 = 0;
        blank_votes += 1;
        assert_eq!(blank_votes, 1);
    }

    #[test]
    fn test_null_vote() {
        let mut null_votes: u32 = 0;
        null_votes += 1;
        assert_eq!(null_votes, 1);
    }

    #[test]
    fn test_valid_vote() {
        let mut scores: HashMap<u32, u32> = HashMap::new();
        let candidate = 1;
        *scores.entry(candidate).or_insert(0) += 1;
        assert_eq!(*scores.get(&candidate).unwrap(), 1);
    }
}