use std::error::Error;
use std::path::PathBuf;

use rocksdb::{WriteBatch, DB};

use crate::traits::{Database, Decode, Encode, Exception, Key};
use crate::tree::tree_node::TreeNode;
use std::marker::PhantomData;

impl From<rocksdb::Error> for Exception {
    #[inline]
    fn from(error: rocksdb::Error) -> Self {
        Self::new(&error.to_string())
    }
}

pub struct RocksDB {
    db: DB,
    pending_inserts: Option<WriteBatch>,
}

impl RocksDB {
    #[inline]
    pub fn new(db: DB) -> Self {
        Self {
            db,
            pending_inserts: Some(WriteBatch::default()),
        }
    }
}

impl<const LENGTH: usize> Database<LENGTH> for RocksDB
where
    TreeNode<LENGTH>: Encode + Decode,
{
    type NodeType = TreeNode<LENGTH>;
    type EntryType = (usize, usize);

    #[inline]
    fn open(path: &PathBuf) -> Result<Self, Exception> {
        Ok(Self::new(DB::open_default(path)?))
    }

    #[inline]
    fn get_node(&self, key: Key<LENGTH>) -> Result<Option<Self::NodeType>, Exception> {
        if let Some(buffer) = self.db.get(&key)? {
            Ok(Some(Self::NodeType::decode(buffer.as_ref())?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn insert(&mut self, key: Key<LENGTH>, value: Self::NodeType) -> Result<(), Exception> {
        let serialized = value.encode()?;
        if let Some(wb) = &mut self.pending_inserts {
            wb.put(key, serialized);
        } else {
            let mut wb = WriteBatch::default();
            wb.put(key, serialized);
            self.pending_inserts = Some(wb);
        }
        Ok(())
    }

    #[inline]
    fn remove(&mut self, key: &Key<LENGTH>) -> Result<(), Exception> {
        Ok(self.db.delete(key)?)
    }

    #[inline]
    fn batch_write(&mut self) -> Result<(), Exception> {
        if let Some(wb) = self.pending_inserts.replace(WriteBatch::default()) {
            self.db.write(wb)?;
        }
        self.pending_inserts = None;
        Ok(())
    }
}
