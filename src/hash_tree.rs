#[cfg(not(any(feature = "use_hashbrown")))]
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "use_hashbrown")]
use hashbrown::HashMap;

use crate::merkle_bit::{BinaryMerkleTreeResult, MerkleBIT};
use crate::traits::{Decode, Encode};
use crate::tree::tree_branch::TreeBranch;
use crate::tree::tree_data::TreeData;
use crate::tree::tree_leaf::TreeLeaf;
use crate::tree::tree_node::TreeNode;
use crate::tree_db::HashTreeDB;
use crate::tree_hasher::TreeHasher;

/// Internal type alias for the underlying tree.
type Tree<ValueType, const LENGTH: usize,> = MerkleBIT<
    HashTreeDB<LENGTH>,
    TreeBranch<LENGTH>,
    TreeLeaf<LENGTH>,
    TreeData,
    TreeNode<LENGTH>,
    TreeHasher,
    ValueType,
    LENGTH
>;

/// A `MerkleBIT` implemented with a `HashMap`.  Can be used for quickly storing items in memory, though
/// larger sets of items should be stored on disk or over the network in a real database.
pub struct HashTree<ValueType, const LENGTH: usize>
where
    ValueType: Encode + Decode
{
    /// The underlying tree.  The type requirements have already been implemented for easy use.
    tree: Tree<ValueType, LENGTH>,
}

impl<ValueType, const LENGTH: usize> HashTree<ValueType, LENGTH>
where
    ValueType: Encode + Decode
{
    /// Creates a new `HashTree`.  `depth` indicates the maximum depth of the tree.
    /// # Errors
    /// None.
    #[inline]
    pub fn new(depth: usize) -> BinaryMerkleTreeResult<Self> {
        let path = PathBuf::new();
        let tree = MerkleBIT::new(&path, depth)?;
        Ok(Self { tree })
    }

    /// Creates a new `HashTree`.  This method exists for conforming with the general API for the `MerkleBIT`
    /// and does not need to be used (except for compatibility).  Prefer `new` when possible.
    /// # Errors
    /// None.
    #[inline]
    pub fn open(path: &PathBuf, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let tree = MerkleBIT::new(path, depth)?;
        Ok(Self { tree })
    }

    /// Gets the values associated with `keys` from the tree.
    /// # Errors
    /// `Exception` generated if the `get` encounters an invalid state during tree traversal.
    #[inline]
    pub fn get(
        &self,
        root_hash: &[u8; LENGTH],
        keys: &mut [[u8; LENGTH]],
    ) -> BinaryMerkleTreeResult<HashMap<[u8; LENGTH], Option<ValueType>>> {
        self.tree.get(root_hash, keys)
    }

    /// Inserts elements into the tree.  Using `previous_root` specifies that the insert depends on
    /// the state from the previous root, and will update references accordingly.
    /// # Errors
    /// `Exception` generated if the `insert` encounters an invalid state during tree traversal.
    #[inline]
    pub fn insert(
        &mut self,
        previous_root: Option<&[u8; LENGTH]>,
        keys: &mut [[u8; LENGTH]],
        values: &[ValueType],
    ) -> BinaryMerkleTreeResult<[u8; LENGTH]> {
        self.tree.insert(previous_root, keys, values)
    }

    /// Removes a root from the tree.  This will remove all elements with less than two references
    /// under the given root.
    /// # Errors
    /// `Exception` generated if the `remove` encounters an invalid state during tree traversal.
    #[inline]
    pub fn remove(&mut self, root_hash: &[u8; LENGTH]) -> BinaryMerkleTreeResult<()> {
        self.tree.remove(root_hash)
    }

    /// Generates an inclusion proof for the given key at the specified root.
    /// # Errors
    /// `Exception` generated if an invalid state is encountered during tree traversal
    #[inline]
    pub fn generate_inclusion_proof(
        &self,
        root: &[u8; LENGTH],
        key: [u8; LENGTH],
    ) -> BinaryMerkleTreeResult<Vec<([u8; LENGTH], bool)>> {
        self.tree.generate_inclusion_proof(root, key)
    }

    /// Verifies an inclusion proof with the given root, key, and value.
    /// # Errors
    /// `Exception` generated if the given proof is invalid.
    #[inline]
    pub fn verify_inclusion_proof(
        root: &[u8; LENGTH],
        key: [u8; LENGTH],
        value: &ValueType,
        proof: &[([u8; LENGTH], bool)],
    ) -> BinaryMerkleTreeResult<()> {
        Tree::verify_inclusion_proof(root, key, value, proof)
    }

    /// Gets a single item out of the tree.
    /// # Errors
    /// `Exception` generated if the `get_one` encounters an invalid state during tree traversal.
    #[inline]
    pub fn get_one(
        &self,
        root: &[u8; LENGTH],
        key: &[u8; LENGTH],
    ) -> BinaryMerkleTreeResult<Option<ValueType>> {
        self.tree.get_one(root, key)
    }

    /// Inserts a single item into the tree.
    /// # Errors
    /// `Exception` generated if the `insert_one` encounters an invalid state during tree traversal.
    #[inline]
    pub fn insert_one(
        &mut self,
        previous_root: Option<&[u8; LENGTH]>,
        key: &[u8; LENGTH],
        value: &ValueType,
    ) -> BinaryMerkleTreeResult<[u8; LENGTH]> {
        self.tree.insert_one(previous_root, key, value)
    }
}
