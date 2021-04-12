use std::collections::hash_map::HashMap;
use std::path::Path;

use crate::traits::{Database, Exception, Key};
use crate::tree::tree_node::TreeNode;

/// A database consisting of a `HashMap`.
pub struct HashDB<const LENGTH: usize> {
    /// The internal `HashMap` for storing nodes.
    map: HashMap<Key<LENGTH>, TreeNode<LENGTH>>,
}

impl<const LENGTH: usize> HashDB<LENGTH> {
    /// Creates a new `HashDB`.
    #[inline]
    #[must_use]
    pub const fn new(map: HashMap<Key<LENGTH>, TreeNode<LENGTH>>) -> Self {
        Self { map }
    }
}

impl<const LENGTH: usize> Database<LENGTH> for HashDB<LENGTH> {
    type NodeType = TreeNode<LENGTH>;
    type EntryType = (Key<LENGTH>, Vec<u8>);

    #[inline]
    fn open(_path: &Path) -> Result<Self, Exception> {
        Ok(Self::new(HashMap::new()))
    }

    #[inline]
    fn get_node(&self, key: Key<LENGTH>) -> Result<Option<Self::NodeType>, Exception> {
        self.map.get(&key).map_or(Ok(None), |m| {
            let node = m.clone();
            Ok(Some(node))
        })
    }

    #[inline]
    fn insert(&mut self, key: Key<LENGTH>, value: Self::NodeType) -> Result<(), Exception> {
        self.map.insert(key, value);
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &[u8; LENGTH]) -> Result<(), Exception> {
        self.map.remove(key);
        Ok(())
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}
