use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "use_digest")]
use digest::Digest;

#[cfg(feature = "use_serde")]
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
#[cfg(feature = "use_serde")]
use serde::ser::SerializeTuple;
#[cfg(feature = "use_serde")]
use serde::{Deserializer, Serializer};
use std::convert::{Infallible, TryFrom};
#[cfg(feature = "use_serde")]
use std::fmt;
#[cfg(feature = "use_serde")]
use std::marker::PhantomData;
use std::num::TryFromIntError;

/// The required interface for structs representing a hasher.
pub trait Hasher<const LENGTH: usize> {
    /// The type of hasher.
    type HashType;
    /// Creates a new `HashType`.
    fn new(size: usize) -> Self::HashType;
    /// Adds data to be hashed.
    fn update(&mut self, data: &[u8]);
    /// Outputs the hash from updated data.
    fn finalize(self) -> Key<LENGTH>;
}

#[cfg(feature = "use_digest")]
impl<T, const LENGTH: usize> Hasher<LENGTH> for T
where
    T: Digest,
{
    type HashType = T;

    fn new(_size: usize) -> Self::HashType {
        Self::HashType::new()
    }

    fn update(&mut self, data: &[u8]) {
        self.update(data);
    }

    fn finalize(self) -> Key<LENGTH> {
        let mut finalized = [0; LENGTH];
        let result = self.finalize();
        let mut size = finalized.as_ref().len();
        if size > result.len() {
            size = result.len();
        }
        finalized.as_mut()[..size].copy_from_slice(&result[..size]);
        finalized
    }
}

/// The required interface for structs representing branches in the tree.
pub trait Branch<const LENGTH: usize> {
    /// Creates a new `Branch`.
    fn new() -> Self;
    /// Gets the count of leaves beneath this node.
    fn get_count(&self) -> u64;
    /// Gets the location of the zero branch beneath this node.
    fn get_zero(&self) -> &Key<LENGTH>;
    /// Gets the location of the one branch beneath this node.
    fn get_one(&self) -> &Key<LENGTH>;
    /// Gets the index on which to split keys when traversing this node.
    fn get_split_index(&self) -> usize;
    /// Gets the associated key with this node.
    fn get_key(&self) -> &Key<LENGTH>;
    /// Sets the count of leaves below this node.
    fn set_count(&mut self, count: u64);
    /// Sets the location of the zero branch beneath this node.
    fn set_zero(&mut self, zero: Key<LENGTH>);
    /// Sets the location of the one branch beneath this node..
    fn set_one(&mut self, one: Key<LENGTH>);
    /// Sets the index on which to split keys when traversing this node.
    fn set_split_index(&mut self, index: usize);
    /// Sets the associated key for this node.
    fn set_key(&mut self, key: Key<LENGTH>);
    /// Decomposes the `Branch` into its constituent parts.
    fn decompose(self) -> (u64, Key<LENGTH>, Key<LENGTH>, usize, Key<LENGTH>);
}

/// The required interface for structs representing leaves in the tree.
pub trait Leaf<const LENGTH: usize> {
    /// Creates a new `Leaf` node.
    fn new() -> Self;
    /// Gets the associated key with this node.
    fn get_key(&self) -> &Key<LENGTH>;
    /// Gets the location of the `Data` node.
    fn get_data(&self) -> &Key<LENGTH>;
    /// Sets the associated key with this node.
    fn set_key(&mut self, key: Key<LENGTH>);
    /// Sets the location of the `Data` node.
    fn set_data(&mut self, data: Key<LENGTH>);
    /// Decomposes the `Leaf` into its constituent parts.
    fn decompose(self) -> (Key<LENGTH>, Key<LENGTH>);
}

/// The required interface for structs representing data stored in the tree.
pub trait Data {
    /// Creates a new `Data` node.
    fn new() -> Self;
    /// Gets the value for the `Data` node.
    fn get_value(&self) -> &[u8];
    /// Sets the value for the `Data` node.
    fn set_value(&mut self, value: &[u8]);
}

/// The required interface for structs representing nodes in the tree.
pub trait Node<BranchType, LeafType, DataType, const LENGTH: usize>
where
    BranchType: Branch<LENGTH>,
    LeafType: Leaf<LENGTH>,
    DataType: Data,
{
    /// Creates a new `Node`.
    fn new(node_variant: NodeVariant<BranchType, LeafType, DataType, LENGTH>) -> Self;
    /// Gets the number of references to this node.
    fn get_references(&self) -> u64;
    /// Decomposes the struct into its inner type.
    fn get_variant(self) -> NodeVariant<BranchType, LeafType, DataType, LENGTH>;
    /// Sets the number of references to this node.
    fn set_references(&mut self, references: u64);
    /// Sets the node to contain a `Branch` node.  Mutually exclusive with `set_data` and `set_leaf`.
    fn set_branch(&mut self, branch: BranchType);
    /// Sets the node to contain a `Leaf` node.  Mutually exclusive with `set_data` and `set_branch`.
    fn set_leaf(&mut self, leaf: LeafType);
    /// Sets the node to contain a `Data` node.  Mutually exclusive with `set_leaf` and `set_branch`.
    fn set_data(&mut self, data: DataType);
}

/// Contains the distinguishing data from the node
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(any(feature = "use_serde",), derive(Serialize, Deserialize))]
pub enum NodeVariant<BranchType, LeafType, DataType, const LENGTH: usize>
where
    BranchType: Branch<LENGTH>,
    LeafType: Leaf<LENGTH>,
    DataType: Data,
{
    /// Variant containing a `Branch` node.
    Branch(BranchType),
    /// Variant containing a `Leaf` node.
    Leaf(LeafType),
    /// Variant containing a `Data` node.
    Data(DataType),
}

/// This trait defines the required interface for connecting a storage mechanism to the `MerkleBIT`.
pub trait Database<const LENGTH: usize> {
    /// The type of node to insert into the database.
    type NodeType;
    /// The type of entry for insertion.  Primarily for convenience and tracking what goes into the database.
    type EntryType;
    /// Opens an existing `Database`.
    /// # Errors
    /// `Exception` generated if the `open` does not succeed.
    fn open(path: &Path) -> Result<Self, Exception>
    where
        Self: Sized;
    /// Gets a value from the database based on the given key.
    /// # Errors
    /// `Exception` generated if the `get_node` does not succeed.
    fn get_node(&self, key: Key<LENGTH>) -> Result<Option<Self::NodeType>, Exception>;
    /// Queues a key and its associated value for insertion to the database.
    /// # Errors
    /// `Exception` generated if the `insert` does not succeed.
    fn insert(&mut self, key: Key<LENGTH>, node: Self::NodeType) -> Result<(), Exception>;
    /// Removes a key and its associated value from the database.
    /// # Errors
    /// `Exception` generated if the `remove` does not succeed.
    fn remove(&mut self, key: &Key<LENGTH>) -> Result<(), Exception>;
    /// Confirms previous inserts and writes the changes to the database.
    /// # Errors
    /// `Exception` generated if the `batch_write` does not succeed.
    fn batch_write(&mut self) -> Result<(), Exception>;
}

/// This trait must be implemented to allow a struct to be serialized.
pub trait Encode {
    /// Encodes a struct into bytes.
    /// # Errors
    /// `Exception` generated when the method encoding the structure fails.
    fn encode(&self) -> Result<Vec<u8>, Exception>;
}

impl Encode for Vec<u8> {
    #[inline]
    fn encode(&self) -> Result<Self, Exception> {
        Ok(self.clone())
    }
}

/// Alias for handling keys of data and within the tree.
pub type Key<const LENGTH: usize> = [u8; LENGTH];

impl<const LENGTH: usize> Encode for Key<LENGTH> {
    #[inline]
    fn encode(&self) -> Result<Vec<u8>, Exception> {
        Ok(self.to_vec())
    }
}

impl<const LENGTH: usize> Decode for Key<LENGTH> {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<Self, Exception>
    where
        Self: Sized,
    {
        return if buffer.len() > LENGTH {
            let mut buf = [0_u8; LENGTH];
            buf.copy_from_slice(&buffer[..LENGTH]);
            Ok(buf)
        } else {
            let buf: [u8; LENGTH] = match <[u8; LENGTH]>::try_from(buffer) {
                Ok(b) => b,
                Err(_) => {
                    return Err(Exception::new(
                        format!("Failed to convert buffer to length {} array", LENGTH).as_str(),
                    ))
                }
            };
            Ok(buf)
        };
    }
}

#[cfg(feature = "use_serde")]
pub trait SerdeHelper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Self: Sized;
}

#[cfg(feature = "use_serde")]
impl<const LENGTH: usize> SerdeHelper for Key<LENGTH> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.len())?;
        for elem in &self[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
        Self: Sized,
    {
        let visitor = ArrayVisitor {
            element: PhantomData,
            length: PhantomData,
        };
        deserializer.deserialize_tuple(LENGTH, visitor)
    }
}

#[cfg(feature = "use_serde")]
struct ArrayVisitor<T, const LENGTH: usize> {
    element: PhantomData<T>,
    length: PhantomData<[usize; LENGTH]>,
}

#[cfg(feature = "use_serde")]
impl<'de, T, const LENGTH: usize> Visitor<'de> for ArrayVisitor<T, LENGTH>
where
    T: Default + Copy + Deserialize<'de>,
{
    type Value = [T; LENGTH];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("an array of length {}", LENGTH))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<[T; LENGTH], A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut arr = [T::default(); LENGTH];
        for i in 0..LENGTH {
            arr[i] = seq
                .next_element()?
                .ok_or_else(|| SerdeError::invalid_length(i, &self))?;
        }
        Ok(arr)
    }
}

/// This trait must be implemented to allow an arbitrary sized buffer to be deserialized.
/// # Errors
/// `Exception` generated when the buffer fails to be decoded to the target type.
pub trait Decode {
    /// Decodes bytes into a `Sized` struct.
    /// # Errors
    /// `Exception` generated when the buffer fails to be decoded to the target type.
    fn decode(buffer: &[u8]) -> Result<Self, Exception>
    where
        Self: Sized;
}

impl Decode for Vec<u8> {
    #[inline]
    fn decode(buffer: &[u8]) -> Result<Self, Exception> {
        Ok(buffer.to_vec())
    }
}

/// A generic error that implements `Error`.
/// Mostly intended to be used to standardize errors across the crate.
#[derive(Debug)]
pub struct Exception {
    /// The details of an exception
    details: String,
}

impl Exception {
    /// Creates a new `Exception`.
    #[inline]
    #[must_use]
    pub fn new(details: &str) -> Self {
        Self {
            details: details.to_owned(),
        }
    }
}

impl Display for Exception {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.details)
    }
}

impl Error for Exception {
    #[inline]
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<Infallible> for Exception {
    #[inline]
    fn from(_inf: Infallible) -> Self {
        Self::new("Infallible")
    }
}

impl From<TryFromIntError> for Exception {
    #[inline]
    fn from(err: TryFromIntError) -> Self {
        Self::new(&err.to_string())
    }
}
