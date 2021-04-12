use crate::traits::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher as DefaultHasherTrait;

impl<const LENGTH: usize> Hasher<LENGTH> for DefaultHasher {
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        Self::new()
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        Self::write(self, data)
    }

    #[inline]
    fn finalize(self) -> [u8; LENGTH] {
        let value = Self::finish(&self).to_le_bytes();
        let mut v = [0; LENGTH];
        if LENGTH >= 8 {
            v.as_mut()[..8].copy_from_slice(&value);
        } else {
            v.as_mut()[..LENGTH].copy_from_slice(&value[..LENGTH]);
        }

        v
    }
}
