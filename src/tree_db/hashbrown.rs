use std::path::PathBuf;

use hashbrown::HashMap;

use crate::traits::{Database, Exception, Key};
use crate::tree::tree_node::TreeNode;

pub struct HashDB<const LENGTH: usize> {
    map: HashMap<Key<LENGTH>, TreeNode<LENGTH>>,
}

impl<const LENGTH: usize> HashDB<LENGTH> {
    #[inline]
    pub fn new(map: HashMap<Key<LENGTH>, TreeNode<LENGTH>>) -> Self {
        Self { map }
    }
}

impl<const LENGTH: usize> Database<LENGTH> for HashDB<LENGTH> {
    type NodeType = TreeNode<LENGTH>;
    type EntryType = (Vec<u8>, TreeNode<LENGTH>);

    #[inline]
    fn open(_path: &PathBuf) -> Result<Self, Exception> {
        Ok(Self::new(HashMap::new()))
    }

    #[inline]
    fn get_node(&self, key: Key<LENGTH>) -> Result<Option<Self::NodeType>, Exception> {
        if let Some(m) = self.map.get(&key) {
            let node = m.clone();
            Ok(Some(node))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn insert(&mut self, key: Key<LENGTH>, value: Self::NodeType) -> Result<(), Exception> {
        self.map.insert(key, value);
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &Key<LENGTH>) -> Result<(), Exception> {
        self.map.remove(key);
        Ok(())
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}
