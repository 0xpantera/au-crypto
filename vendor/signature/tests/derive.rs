//! Tests for code generated by `signature_derive`

#![cfg(all(feature = "derive-preview", feature = "hazmat-preview"))]

use digest::{generic_array::GenericArray, Digest, OutputSizeUser};
use hex_literal::hex;
use sha2::Sha256;
use signature::{
    hazmat::{PrehashSigner, PrehashVerifier},
    DigestSigner, DigestVerifier, Error, PrehashSignature, Signature, Signer, Verifier,
};

/// Test vector to compute SHA-256 digest of
const INPUT_STRING: &[u8] = b"abc";

/// Expected SHA-256 digest for the input string
const INPUT_STRING_DIGEST: [u8; 32] =
    hex!("ba7816bf 8f01cfea 414140de 5dae2223 b00361a3 96177a9c b410ff61 f20015ad");

/// Dummy signature which just contains a digest output
#[derive(Debug)]
struct DummySignature(GenericArray<u8, <Sha256 as OutputSizeUser>::OutputSize>);

impl Signature for DummySignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(DummySignature(GenericArray::clone_from_slice(
            bytes.as_ref(),
        )))
    }
}

impl AsRef<[u8]> for DummySignature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl PrehashSignature for DummySignature {
    type Digest = Sha256;
}

/// Dummy signer which just returns the message digest as a `DummySignature`
#[derive(Signer, DigestSigner, Default)]
struct DummySigner {}

impl PrehashSigner<DummySignature> for DummySigner {
    fn sign_prehash(&self, prehash: &[u8]) -> signature::Result<DummySignature> {
        DummySignature::from_bytes(prehash)
    }
}

/// Dummy verifier which ensures the `DummySignature` digest matches the
/// expected value.
///
/// Panics (via `assert_eq!`) if the value is not what is expected.
#[derive(Verifier, DigestVerifier, Default)]
struct DummyVerifier {}

impl PrehashVerifier<DummySignature> for DummyVerifier {
    fn verify_prehash(&self, prehash: &[u8], signature: &DummySignature) -> signature::Result<()> {
        assert_eq!(signature.as_ref(), prehash);
        Ok(())
    }
}

#[test]
fn derived_signer_impl() {
    let sig: DummySignature = DummySigner::default().sign(INPUT_STRING);
    assert_eq!(sig.as_ref(), INPUT_STRING_DIGEST.as_ref())
}

#[test]
fn derived_verifier_impl() {
    let sig: DummySignature = DummySigner::default().sign(INPUT_STRING);
    assert!(DummyVerifier::default().verify(INPUT_STRING, &sig).is_ok());
}
