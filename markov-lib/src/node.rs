use fxhash::FxHashMap;
use rand::Rng;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node {
    weights: FxHashMap<usize, usize>,
    total_weight: usize,
    pub(crate) is_sentence_end: bool,
}

impl Node {
    pub fn get_random(&self) -> Option<usize> {
        self.get_weighted(self.roll())
    }

    pub fn add(&mut self, idx: usize) {
        self.add_weight(idx, 1);
    }

    fn get_weighted(&self, target_weight: usize) -> Option<usize> {
        let mut acc = 0;
        self.weights
            .iter()
            .find(|(_, weight)| {
                acc += **weight;
                acc > target_weight
            })
            .map(|(idx, _)| *idx)
    }

    fn add_weight(&mut self, idx: usize, weight: usize) {
        self.total_weight += weight;
        self.weights
            .entry(idx)
            .and_modify(|w| *w += weight)
            .or_insert(weight);
    }

    fn roll(&self) -> usize {
        if self.total_weight > 0 {
            rand::thread_rng().gen_range(0..self.total_weight)
        } else {
            0
        }
    }

    pub fn weight_of(&self, idx: usize) -> Option<usize> {
        self.weights.get(&idx).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn add() {
        let mut node = Node::default();
        node.add(1);
        node.add(1);
        node.add(1);
        node.add(2);
        assert_eq!(node.weights.get(&1), Some(&3));
        assert_eq!(node.weights.get(&2), Some(&1));
        assert_eq!(node.weights.get(&3), None);
        assert_eq!(node.total_weight, 4);
    }

    #[test]
    fn weighted_add_and_get() {
        let mut node = Node::default();
        node.add_weight(1, 2);
        node.add_weight(2, 2);
        node.add_weight(3, 1);
        assert_eq!(node.get_weighted(4), Some(2));
        assert_eq!(node.get_weighted(5), Some(3));
        assert_eq!(node.get_weighted(6), None);
        assert_eq!(node.total_weight, 5);
    }

    #[test]
    fn get_random() {
        let mut node = Node::default();
        assert_eq!(node.get_random(), None);
        node.add(10);
        assert_eq!(node.get_random(), Some(10));
    }
}
