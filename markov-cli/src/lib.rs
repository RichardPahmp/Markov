pub mod commands;
use std::{fs, io, path::Path};

use anyhow::Result;
use markov_lib::Chain;

pub fn write_chain(path: &Path, chain: &Chain) -> Result<()> {
    Ok(fs::write(path, bincode::serialize(&chain)?)?)
}

pub fn load_chain(path: &Path) -> Result<Chain> {
    deserialize_chain(&load_chainfile(path)?)
}

pub fn load_chainfile(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn deserialize_chain(bytes: &[u8]) -> Result<Chain> {
    Ok(bincode::deserialize(bytes)?)
}
