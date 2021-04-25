use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

pub struct KeccakHasher(Keccak);

impl<const LENGTH: usize> crate::traits::Hasher<LENGTH> for KeccakHasher
{
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        let hasher = Keccak::v256();
        Self(hasher)
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    #[inline]
    fn finalize(self) -> [u8; LENGTH] {
        let mut res = [0; LENGTH];
        self.0.finalize(res.as_mut());
        res
    }
}
