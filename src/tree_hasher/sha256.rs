use openssl::sha::Sha256;

pub struct Sha256Hasher(Sha256);

impl<const LENGTH: usize> crate::traits::Hasher<LENGTH> for Sha256Hasher
{
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        let hasher = Sha256::new();
        Self(hasher)
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data)
    }

    #[inline]
    fn finalize(self) -> [u8; LENGTH] {
        let mut v = [0; LENGTH];
        let value = self.0.finish();
        v.as_mut()[..LENGTH].copy_from_slice(&value[..LENGTH]);
        v
    }
}
