use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use markov_cli::{commands::generate, load_chain, load_chainfile, write_chain};
use markov_lib::Chain;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Where to load/store the chain
    #[arg(short = 'f', long = "file")]
    chainfile: PathBuf,

    /// Start from an empty chain
    #[arg(short, long)]
    clean: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create an empty chain file
    New,
    /// Load string data from the given file and feed it to the chain
    Load {
        filename: PathBuf,
    },
    /// Generate a random sentence from the chain
    Generate {
        /// How many sentences to generate
        #[arg(short, long, default_value_t = 5)]
        number: usize,
        /// Start each sentence from a given word
        starting_word: Option<String>,
        /// Save the output to a file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Debug,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::New => {
            let chain = Chain::default();
            write_chain(&args.chainfile, &chain)?
        }
        Commands::Load { filename } => {
            let mut chain = load_chain(&args.chainfile)?;
            let data = fs::read_to_string(filename)?;
            chain.feed(&data);
            write_chain(&args.chainfile, &chain)?;
        }
        Commands::Generate {
            starting_word,
            number,
            output,
        } => {
            let chain = load_chain(&args.chainfile)?;
            let starting_word = starting_word.unwrap();
            for _ in 0..number {
                println!("{}", generate(&chain, Some(&starting_word)));
            }
        }
        Commands::Debug => {
            let chain = load_chain(&args.chainfile)?;
            dbg!(chain);
        }
    }
    Ok(())
}
