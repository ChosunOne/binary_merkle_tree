use tiny_keccak::Hasher;
use tiny_keccak::Sha3;

pub struct Sha3Hasher(Sha3);

impl<const LENGTH: usize> crate::traits::Hasher<LENGTH> for Sha3Hasher
{
    type HashType = Self;

    #[inline]
    fn new(_size: usize) -> Self {
        let hasher = Sha3::v256();
        Self(hasher)
    }

    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    #[inline]
    fn finalize(self) -> [u8; LENGTH] {
        let mut res =[0; LENGTH];
        self.0.finalize(res.as_mut());
        res
    }
}
