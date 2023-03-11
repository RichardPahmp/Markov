use crate::Chain;

pub struct ChainIterator<'a> {
    chain: &'a Chain,
    idx: Option<usize>,
}

impl<'a> ChainIterator<'a> {
    pub(crate) fn new(chain: &'a Chain, idx: Option<usize>) -> Self {
        Self { chain, idx }
    }
}

impl<'a> Iterator for ChainIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let (word, node) = self.chain.get_full(self.idx?)?;
        self.idx = node.get_random();
        Some(word)
    }
}
