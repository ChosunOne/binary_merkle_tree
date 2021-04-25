use fxhash::FxHasher;
use std::hash::Hasher;

impl<const LENGTH: usize> crate::traits::Hasher<LENGTH> for FxHasher
{
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        Self::default()
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        Hasher::write(self, data)
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
