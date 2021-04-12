use crate::traits::{Branch, Data, Leaf};

/// Represents a position in the tree during tree traversal.
pub struct TreeCell<'a, NodeType, const LENGTH: usize>
{
    /// The location of the node being traversed.
    pub location: [u8; LENGTH],
    /// The keys traversing this part of the tree.
    pub keys: &'a [[u8; LENGTH]],
    /// The node currently being traversed.
    pub node: NodeType,
    /// The depth of the traversal in the tree.
    pub depth: usize,
}

impl<'a, NodeType, const LENGTH: usize> TreeCell<'a, NodeType, LENGTH>
{
    /// Creates a new `TreeCell`.
    #[inline]
    pub fn new<BranchType, LeafType, DataType>(
        location: [u8; LENGTH],
        keys: &'a [[u8; LENGTH]],
        node: NodeType,
        depth: usize,
    ) -> Self
    where
        BranchType: Branch<LENGTH>,
        LeafType: Leaf<LENGTH>,
        DataType: Data,
    {
        Self {
            location,
            keys,
            node,
            depth,
        }
    }
}
