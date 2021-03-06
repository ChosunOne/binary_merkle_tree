// Clippy configurations
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::implicit_return)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::module_name_repetitions)]

#![forbid(unsafe_code)]

//! # Merkle Binary Indexed Tree
//! ## Introduction
//! This module implements [`MerkleBIT`](merkle_bit/struct.MerkleBIT.html) with an attached storage module.  The implemented [`HashTree`](hash_tree/struct.HashTree.html)
//! and [`RocksTree`](rocks_tree/struct.RocksTree.html) structures allow use with persistence in memory and storage respectively.  Write
//! operations are batched together and committed at the end of each insert op.  The [`MerkleBit`](merkle_bit/struct.MerkleBIT.html) API
//! abstracts all actions related to maintaining and updating the storage tree.  The public APIs are
//! * [`new`](merkle_bit/struct.MerkleBIT.html#method.new)
//! * [`from_db`](merkle_bit/struct.MerkleBIT.html#method.from_db)
//! * [`get`](merkle_bit/struct.MerkleBIT.html#method.get)
//! * [`insert`](merkle_bit/struct.MerkleBIT.html#method.insert)
//! * [`remove`](merkle_bit/struct.MerkleBIT.html#method.remove)
//! * [`generate_inclusion_proof`](merkle_bit/struct.MerkleBIT.html#method.generate_inclusion_proof)
//! * [`get_one`](merkle_bit/struct.MerkleBIT.html#method.get_one)
//! * [`insert_one`](merkle_bit/struct.MerkleBIT.html#method.insert_one)
//! * and the associated function [`verify_inclusion_proof`](merkle_bit/struct.MerkleBIT.html#method.verify_inclusion_proof).
//!
//! After each call to either `insert` or `insert_one`, a new root hash will be created which can be
//! later used to access the inserted items.
//!
//! ## Internal Structure
//! Internally, the `MerkleBit` is composed of a collection of trait structs which implement the
//! [`Branch`](traits/trait.Branch.html), [`Leaf`](traits/trait.Leaf.html), and [`Data`](traits/trait.Data.html) nodes of the tree.
//!
//! A `Branch` node contains first a `split_index`
//! which indicates which bit of a given hash should be used to traverse the tree.  This is an optimisation
//! that makes this a *sparse* merkle tree.  It then contains pointers
//! to either the `one` side of the tree or the `zero` side of the tree.  Additionally, a branch contains
//! a copy of the `key` used during creation to determine if a branch should be inserted before it, and
//! a `count` of the nodes under that branch.
//!
//! A `Leaf` node contains an associated `key` for comparison, and a pointer to a `Data` node for retrieving
//! information regarding access to the data.  This is separate from the `Data` node for the purpose of only
//! accessing data information if data should be retrieved.
//!
//! A `Data` node contains the actual information to be retrieved.  `Data` nodes can be arbitrary in size
//! and the only restriction is that the data must be serializable and deserializable.
//!
//! To illustrate these concepts, please refer to the diagram below:
//!
//! ```text
//!                                                 ----------------------
//!                                 branch  --->    | split_index: usize |
//!                                   |             | zero: [u8]         |
//!       ----------------           / \            | one:  [u8]         |
//!       | key: [u8]    |          /   \           | count: u64         |
//!       | data: [u8]   | <----  leaf   leaf       | key:   [u8]        |
//!       ----------------         |                ----------------------
//!                                |
//!                                V
//!                              data
//!                                |
//!                                V
//!                         ------------------
//!                         | value: Vec<u8> |
//!                         ------------------
//! ```
//!
//! The `MerkleBIT` can be extended to support a wide variety of backend storage solutions given that
//! you make implementations for the `Branch`, `Leaf`, and `Data` traits.

/// Defines constants for the tree.
pub mod constants;
/// An implementation of the `MerkleBIT` with a `HashMap` backend database.
pub mod hash_tree;
/// Contains the actual operations of inserting, getting, and removing items from a tree.
pub mod merkle_bit;
/// Contains the traits necessary for tree operations
pub mod traits;
/// Contains a collection of structs for representing locations within the tree.
pub mod tree;
/// Contains a collection of structs for implementing tree databases.
pub mod tree_db;
/// Contains a collection of structs for implementing hashing functions in the tree.
pub mod tree_hasher;
/// Contains a collection of useful structs and functions for tree operations.
pub mod utils;

#[cfg(feature = "use_rocksdb")]
/// An implementation of the `MerkleBIT` with a `RocksDB` backend database.
pub mod rocks_tree;
