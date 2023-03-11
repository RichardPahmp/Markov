use clap::Parser;

#[derive(Parser, Debug)]
struct Args {}

pub fn main() {
    let args = Args::parse();
}
