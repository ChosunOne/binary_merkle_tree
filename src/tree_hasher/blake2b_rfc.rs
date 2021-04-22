use blake2_rfc;

#[derive(Clone)]
pub struct Blake2bHasher(blake2_rfc::blake2b::Blake2b);

impl<const LENGTH: usize> crate::traits::Hasher<[u8; LENGTH]> for Blake2bHasher
{
    type HashType = Self;

    #[inline]
    fn new(size: usize) -> Self {
        let hasher = blake2_rfc::blake2b::Blake2b::new(size);
        Self(hasher)
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    #[inline]
    fn finalize(self) -> [u8; LENGTH] {
        let result = self.0.finalize();
        let mut finalized = [0; LENGTH];
        finalized.as_mut().copy_from_slice(result.as_ref());
        finalized
    }
}
