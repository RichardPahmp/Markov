mod chain;
mod iter;
mod node;

pub use chain::Chain;
pub use iter::ChainIterator;

fn is_sentence_end(word: &str) -> bool {
    word.ends_with('.')
}
