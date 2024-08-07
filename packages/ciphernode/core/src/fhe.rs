use fhe::{
    bfv::{BfvParameters, BfvParametersBuilder, SecretKey as FheRsSecretKey},
    mbfv::{CommonRandomPoly, PublicKeyShare as FheRsPublicKeyShare},
};
use fhe_traits::Serialize;
use rand::{CryptoRng, RngCore};
use std::sync::Arc;

// Some loose error/result stuff we can use for this module
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// Define a trait for Rng which we use below
pub trait Rng: RngCore + CryptoRng {}
impl<T: RngCore + CryptoRng> Rng for T {}

/// Wrapped PublicKeyShare. This is wrapped to provide an inflection point
/// as we use this library elsewhere we only implement traits as we need them
/// and avoid exposing underlying structures from fhe.rs
#[derive(Debug, Clone)]
pub struct PublicKeyShare(pub FheRsPublicKeyShare);

impl PublicKeyShare {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<PublicKeyShare> for Vec<u8> {
    fn from(share: PublicKeyShare) -> Vec<u8> {
        share.as_bytes()
    }
}


/// Our wrapped SecretKey
#[derive(PartialEq)] // Avoid adding debugging and copy traits as this is a secret key and we want
                     // Underlying struct is a Box<[i64]> so Copy will do a memory copy although
                     // the key is zeroized on drop
pub struct SecretKey(pub FheRsSecretKey);

/// Fhe is the accessor crate for our Fhe encryption lib. We should use this as an inflection point.
/// Underlying internal types and errors should not be leaked. We should aim to maintain a simple 
/// API in line with our needs not the underlying library and what this does should be pretty
/// lightweight
pub struct Fhe {
    params: Arc<BfvParameters>,
    crp: CommonRandomPoly,
}

impl Fhe {
    pub fn new<R: Rng>(
        rng: &mut R,
        moduli: Vec<u64>,
        degree: usize,
        plaintext_modulus: u64,
    ) -> Result<Fhe> {
        let params = BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?;
        let crp = CommonRandomPoly::new(&params, rng)?;
        Ok(Fhe { params, crp })
    }

    pub fn get_params(&self) -> (&Arc<BfvParameters>, &CommonRandomPoly) {
        (&self.params, &self.crp)
    }

    pub fn generate_keyshare<R: Rng>(&self, rng: &mut R) -> Result<(SecretKey, PublicKeyShare)> {
        let sk_share = FheRsSecretKey::random(&self.params, rng);
        let pk_share = FheRsPublicKeyShare::new(&sk_share, self.crp.clone(), rng)?;
        Ok((SecretKey(sk_share), PublicKeyShare(pk_share)))
    }
}
