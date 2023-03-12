use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use rand::Rng;

use crate::{is_sentence_end, node::Node, ChainIterator};
use itertools::Itertools;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

type IndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chain {
    map: IndexMap<String, Node>,
    starter_words: IndexSet<usize>,
}

impl Chain {
    pub fn feed(&mut self, string: &str) {
        for (current_word, next_word) in string.split_whitespace().tuple_windows() {
            let current_is_end = is_sentence_end(current_word);

            let (next_idx, _) = self.get_or_insert_mut(next_word);
            if current_is_end {
                self.starter_words.insert(next_idx);
            }

            let (_, current_node) = self.get_or_insert_mut(current_word);
            if current_is_end {
                current_node.is_sentence_end = true;
            }
            current_node.add(next_idx);
        }
    }

    fn get_or_insert_mut(&mut self, word: &str) -> (usize, &mut Node) {
        if !self.map.contains_key(word) {
            let entry = self.map.entry(String::from(word));
            (entry.index(), entry.or_default())
        } else {
            let (idx, _, node) = self.map.get_full_mut(word).unwrap();
            (idx, node)
        }
    }

    pub fn index_of(&self, word: &str) -> Option<usize> {
        self.map.get_index_of(word)
    }

    pub fn weight_between(&self, first: &str, second: &str) -> Option<usize> {
        let first = self.map.get(first);
        let second = self.map.get_index_of(second);
        match (first, second) {
            (Some(left), Some(right)) => left.weight_of(right),
            _ => None,
        }
    }

    fn iter(&self, idx: usize) -> ChainIterator<'_> {
        ChainIterator::new(self, idx)
    }

    pub fn generate_sentence(&self, starting_word: Option<&str>) -> String {
        let starting_word = starting_word
            .and_then(|word| self.index_of(word))
            .unwrap_or_else(|| self.get_random_starter_word());
        self.iter(starting_word).join(" ")
    }

    pub(crate) fn get_full(&self, idx: usize) -> Option<(&str, &Node)> {
        self.map.get_index(idx).map(|(k, v)| (k.as_str(), v))
    }

    fn get_random_starter_word(&self) -> usize {
        *self
            .starter_words
            .get_index(rand::thread_rng().gen_range(0..self.starter_words.len()))
            .unwrap()
    }
}

#[cfg(test)]

mod tests {
    use itertools::Itertools;

    use crate::Chain;

    #[test]
    fn feed() {
        let mut chain = Chain::default();
        assert!(chain.index_of("hello").is_none());
        assert!(chain.index_of("world").is_none());
        assert!(chain.index_of("lizer").is_none());
        chain.feed("hello world");
        assert!(chain.index_of("hello").is_some());
        assert!(chain.index_of("world").is_some());
        assert!(chain.index_of("lizer").is_none());
    }

    #[test]
    fn generate_iter() {
        let mut chain = Chain::default();
        chain.feed("henlo stinky lizer");
        assert_eq!(
            vec!["henlo", "stinky", "lizer"],
            chain.generate_iter("henlo").collect::<Vec<_>>(),
        )
    }

    #[test]
    fn generate_empty_iter() {
        let chain = Chain::default();
        assert_eq!(chain.generate_iter("hello").next(), None);
    }

    #[test]
    fn weights() {
        let mut chain = Chain::default();
        chain.feed("henlo stinky");
        chain.feed("henlo stinky");
        chain.feed("henlo lizer");
        chain.feed("henlo boy");
        chain.feed("stinky lizer");
        chain.feed("stinky stinky boy");
        chain.feed("stinky stinky stinky stinky stinky");
        assert_eq!(chain.weight_between("henlo", "stinky"), Some(2));
        assert_eq!(chain.weight_between("henlo", "lizer"), Some(1));
        assert_eq!(chain.weight_between("henlo", "boy"), Some(1));
        assert_eq!(chain.weight_between("stinky", "boy"), Some(1));
        assert_eq!(chain.weight_between("stinky", "lizer"), Some(1));
        assert_eq!(chain.weight_between("stinky", "stinky"), Some(5));
    }

    #[test]
    fn test() {
        let mut chain = Chain::default();
        chain.feed("henlo stinky");
        chain.feed("henlo lizer");
        chain.feed("henlo boy");
        chain.feed("stinky lizer");
        chain.feed("stinky boy");
        chain.feed("stinky stinky");
        for _ in 0..20 {
            println!("{}", chain.generate_iter("henlo").join(" "));
        }
    }

    #[test]
    fn testing() {
        let mut chain = Chain::default();
        chain.feed("stinky boy");
        chain.feed("stinky lizer");
        chain.feed("stinky pinky");
        let mut boy_count = 0;
        let mut lizer_count = 0;
        let mut pinky_count = 0;
        for _ in 0..100000 {
            let string = chain.generate_iter("stinky").skip(1).next().unwrap();
            match string {
                "boy" => boy_count += 1,
                "lizer" => lizer_count += 1,
                "pinky" => pinky_count += 1,
                _ => panic!(),
            }
        }
        println!(
            "boy: {}, lizer: {}, pinky: {}",
            boy_count, lizer_count, pinky_count
        );
    }
}
