#[cfg(feature = "use_bincode")]
use bincode::{deserialize, serialize};
#[cfg(feature = "use_ron")]
use ron;
#[cfg(feature = "use_serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "use_cbor")]
use serde_cbor;
#[cfg(feature = "use_json")]
use serde_json;
#[cfg(feature = "use_pickle")]
use serde_pickle;
#[cfg(feature = "use_yaml")]
use serde_yaml;

#[cfg(feature = "use_serialization")]
use crate::merkle_bit::BinaryMerkleTreeResult;
use crate::traits::{Leaf, Key};
#[cfg(feature = "use_serialization")]
use crate::traits::{Decode, Encode, SerdeHelper};

/// Represents a leaf of the tree.  Holds a pointer to the location of the underlying `Data` node.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "use_serde", derive(Serialize, Deserialize))]
pub struct TreeLeaf<const LENGTH: usize>
{
    /// The associated key with this node.
    #[serde(with = "SerdeHelper")]
    key: Key<LENGTH>,
    /// The location of the `Data` node in the tree.
    #[serde(with = "SerdeHelper")]
    data: Key<LENGTH>,
}

impl<const LENGTH: usize> TreeLeaf<LENGTH>
{
    /// Creates a new `TreeLeaf`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            key: [0; LENGTH],
            data: [0; LENGTH],
        }
    }

    /// Gets the associated key with the node.
    fn get_key(&self) -> &Key<LENGTH> {
        &self.key
    }

    /// Gets the location of the `Data` node from this node.
    fn get_data(&self) -> &Key<LENGTH> {
        &self.data
    }

    /// Sets the associated key with the node.
    fn set_key(&mut self, key: Key<LENGTH>) {
        self.key = key;
    }

    /// Sets the location of the `Data` node.
    fn set_data(&mut self, data: Key<LENGTH>) {
        self.data = data;
    }

    /// Decomposes the `TreeLeaf` into its constituent parts.
    fn decompose(self) -> (Key<LENGTH>, Key<LENGTH>) {
        (self.key, self.data)
    }
}

impl<const LENGTH: usize> Leaf<LENGTH> for TreeLeaf<LENGTH>
{
    /// Creates a new `TreeLeaf`
    #[inline]
    fn new() -> Self {
        Self::new()
    }

    /// Gets the associated key with this node.
    #[inline]
    fn get_key(&self) -> &Key<LENGTH> {
        Self::get_key(self)
    }

    /// Gets the location of the `Data` node.
    #[inline]
    fn get_data(&self) -> &Key<LENGTH> {
        Self::get_data(self)
    }

    /// Sets the associated key with this node.
    #[inline]
    fn set_key(&mut self, key: Key<LENGTH>) {
        Self::set_key(self, key)
    }

    /// Sets the location for the `Data` node.
    #[inline]
    fn set_data(&mut self, data: Key<LENGTH>) {
        Self::set_data(self, data)
    }

    /// Decomposes the struct into its constituent parts.
    #[inline]
    fn decompose(self) -> (Key<LENGTH>, Key<LENGTH>) {
        Self::decompose(self)
    }
}

#[cfg(feature = "use_bincode")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        Ok(serialize(self)?)
    }
}

#[cfg(feature = "use_json")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        let encoded = serde_json::to_string(&self)?;
        Ok(encoded.as_bytes().to_vec())
    }
}

#[cfg(feature = "use_cbor")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        Ok(serde_cbor::to_vec(&self)?)
    }
}

#[cfg(feature = "use_yaml")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        Ok(serde_yaml::to_vec(&self)?)
    }
}

#[cfg(feature = "use_pickle")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        Ok(serde_pickle::to_vec(&self, true)?)
    }
}

#[cfg(feature = "use_ron")]
impl<const LENGTH: usize> Encode for TreeLeaf<LENGTH>
{
    #[inline]
    fn encode(&self) -> BinaryMerkleTreeResult<Vec<u8>> {
        Ok(ron::ser::to_string(&self)?.as_bytes().to_vec())
    }
}

#[cfg(feature = "use_bincode")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        Ok(deserialize(buffer)?)
    }
}

#[cfg(feature = "use_json")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        let decoded_string = String::from_utf8(buffer.to_vec())?;
        let decoded = serde_json::from_str(&decoded_string)?;
        Ok(decoded)
    }
}

#[cfg(feature = "use_cbor")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        Ok(serde_cbor::from_slice(buffer)?)
    }
}

#[cfg(feature = "use_yaml")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        Ok(serde_yaml::from_slice(buffer)?)
    }
}

#[cfg(feature = "use_pickle")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        Ok(serde_pickle::from_slice(buffer)?)
    }
}

#[cfg(feature = "use_ron")]
impl<const LENGTH: usize> Decode for TreeLeaf<LENGTH>
{
    #[inline]
    fn decode(buffer: &[u8]) -> BinaryMerkleTreeResult<Self> {
        Ok(ron::de::from_bytes(buffer)?)
    }
}
