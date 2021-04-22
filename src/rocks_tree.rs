#[cfg(not(any(feature = "use_hashbrown")))]
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "use_hashbrown")]
use hashbrown::HashMap;

use crate::merkle_bit::{BinaryMerkleTreeResult, MerkleBIT};
use crate::traits::{Array, Database, Decode, Encode};
use crate::tree::tree_branch::TreeBranch;
use crate::tree::tree_data::TreeData;
use crate::tree::tree_leaf::TreeLeaf;
use crate::tree::tree_node::TreeNode;
use crate::tree_db::rocksdb::RocksDB;
use crate::tree_hasher::TreeHasher;
#[cfg(feature = "use_serde")]
use serde::de::DeserializeOwned;
#[cfg(feature = "use_serde")]
use serde::Serialize;

/// Internal type alias for the underlying tree.
type Tree<ValueType, const LENGTH: usize> = MerkleBIT<
    RocksDB,
    TreeBranch<LENGTH>,
    TreeLeaf<LENGTH>,
    TreeData,
    TreeNode<LENGTH>,
    TreeHasher,
    ValueType,
    LENGTH
>;

pub struct RocksTree<ValueType, const LENGTH: usize>
where
    ValueType: Encode + Decode,
{
    tree: Tree<ValueType, LENGTH>,
}

impl<ValueType, const LENGTH: usize> RocksTree<ValueType, LENGTH>
where
    ValueType: Encode + Decode,
{
    #[inline]
    pub fn open(path: &PathBuf, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let db = RocksDB::open(path)?;
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(Self { tree })
    }

    #[inline]
    pub fn from_db(db: RocksDB, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(Self { tree })
    }

    #[inline]
    pub fn get(
        &self,
        root_hash: &[u8; LENGTH],
        keys: &mut [[u8; LENGTH]],
    ) -> BinaryMerkleTreeResult<HashMap<[u8; LENGTH], Option<ValueType>>> {
        self.tree.get(root_hash, keys)
    }

    #[inline]
    pub fn get_one(
        &self,
        root: &[u8; LENGTH],
        key: &[u8; LENGTH],
    ) -> BinaryMerkleTreeResult<Option<ValueType>> {
        self.tree.get_one(&root, &key)
    }

    #[inline]
    pub fn insert(
        &mut self,
        previous_root: Option<&[u8; LENGTH]>,
        keys: &mut [[u8; LENGTH]],
        values: &[ValueType],
    ) -> BinaryMerkleTreeResult<[u8; LENGTH]> {
        self.tree.insert(previous_root, keys, values)
    }

    #[inline]
    pub fn insert_one(
        &mut self,
        previous_root: Option<&[u8; LENGTH]>,
        key: &[u8; LENGTH],
        value: &ValueType,
    ) -> BinaryMerkleTreeResult<[u8; LENGTH]> {
        self.tree.insert_one(previous_root, key, value)
    }

    #[inline]
    pub fn remove(&mut self, root_hash: &[u8; LENGTH]) -> BinaryMerkleTreeResult<()> {
        self.tree.remove(root_hash)
    }

    #[inline]
    pub fn generate_inclusion_proof(
        &self,
        root: &[u8; LENGTH],
        key: [u8; LENGTH],
    ) -> BinaryMerkleTreeResult<Vec<([u8; LENGTH], bool)>> {
        self.tree.generate_inclusion_proof(root, key)
    }

    #[inline]
    pub fn verify_inclusion_proof(
        root: &[u8; LENGTH],
        key: [u8; LENGTH],
        value: &ValueType,
        proof: &Vec<([u8; LENGTH], bool)>,
    ) -> BinaryMerkleTreeResult<()> {
        Tree::verify_inclusion_proof(root, key, value, proof)
    }
}
