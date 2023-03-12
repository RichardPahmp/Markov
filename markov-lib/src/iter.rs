use crate::{is_sentence_end, Chain};

pub struct ChainIterator<'a> {
    chain: &'a Chain,
    idx: Option<usize>,
    ended: bool,
}

impl<'a> ChainIterator<'a> {
    pub(crate) fn new(chain: &'a Chain, idx: usize) -> Self {
        Self {
            chain,
            idx: Some(idx),
            ended: false,
        }
    }
}

impl<'a> Iterator for ChainIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        let (word, node) = self.chain.get_full(self.idx?)?;
        if is_sentence_end(word) {
            self.ended = true;
            Some(word)
        } else {
            self.idx = node.get_random();
            Some(word)
        }
    }
}
