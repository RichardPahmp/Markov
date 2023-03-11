use markov_lib::Chain;

pub fn generate(chain: &Chain, starting_word: Option<&str>) -> String {
    chain.generate_sentence(starting_word)
}
