use std::marker::PhantomData;

use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};

pub struct Transcript<F: PrimeField, T: HasherTrait> {
    hasher: T,
    f_element: PhantomData<F>,
}

impl<F: PrimeField, T: HasherTrait> Transcript<F, T> {
    pub fn init(hash_function: T) -> Self {
        Transcript {
            hasher: hash_function,
            f_element: PhantomData,
        }
    }
    pub fn append(&mut self, data: &[u8]) {
        self.hasher.absorb(data);
    }

    pub fn hash(&mut self) -> F {
        let hash = self.hasher.squeeze();
        F::from_be_bytes_mod_order(&hash)
    }
}
// defn: This trait is what we would enforce any hash_fn to conform to
// this is basicaly to have a standard interface
pub trait HasherTrait {
    fn absorb(&mut self, data: &[u8]);
    fn squeeze(&mut self) -> Vec<u8>;
}

impl HasherTrait for Keccak256 {
    fn absorb(&mut self, data: &[u8]) {
        self.update(data);
    }
    fn squeeze(&mut self) -> Vec<u8> {
        let hash = self.finalize_reset().to_vec();
        self.absorb(&hash);
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::BigInteger;

    #[test]
    fn test_hash() {
        let mut transcript = Transcript::<Fq, Keccak256>::init(Keccak256::new());
        transcript.append(b"test data 1");
        transcript.append(Fq::from(7).into_bigint().to_bytes_be().as_slice());
        transcript.append(b"test data 2");
        let hash = transcript.hash();
        let hash2 = transcript.hash();
        let hash3 = transcript.hash();
        assert_ne!(hash,hash2);
        assert_ne!(hash,hash3);
        assert_ne!(hash2,hash3);
    }
}
