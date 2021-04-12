use std::collections::hash_map::HashMap;
use std::path::PathBuf;

use crate::traits::{Database, Exception};
use crate::tree::tree_node::TreeNode;

/// A database consisting of a `HashMap`.
pub struct HashDB<const LENGTH: usize>
{
    /// The internal `HashMap` for storing nodes.
    map: HashMap<[u8; LENGTH], TreeNode<LENGTH>>,
}

impl<const LENGTH: usize> HashDB<LENGTH>
{
    /// Creates a new `HashDB`.
    #[inline]
    #[must_use]
    pub fn new(map: HashMap<[u8; LENGTH], TreeNode<LENGTH>>) -> Self {
        Self { map }
    }
}

impl<const LENGTH: usize> Database<LENGTH> for HashDB<LENGTH>
{
    type NodeType = TreeNode<LENGTH>;
    type EntryType = ([u8; LENGTH], Vec<u8>);

    #[inline]
    fn open(_path: &PathBuf) -> Result<Self, Exception> {
        Ok(Self::new(HashMap::new()))
    }

    #[inline]
    fn get_node(&self, key: [u8; LENGTH]) -> Result<Option<Self::NodeType>, Exception> {
        if let Some(m) = self.map.get(&key) {
            let node = m.clone();
            Ok(Some(node))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn insert(&mut self, key: [u8; LENGTH], value: Self::NodeType) -> Result<(), Exception> {
        self.map.insert(key, value);
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &[u8 ;LENGTH]) -> Result<(), Exception> {
        self.map.remove(key);
        Ok(())
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}
