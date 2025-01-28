use clap::Parser;

/// Configuration structure to hold command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Configuration {
    /// List of candidates
    #[clap(short, long, use_delimiter = true)]
    pub candidates: Vec<String>,
}